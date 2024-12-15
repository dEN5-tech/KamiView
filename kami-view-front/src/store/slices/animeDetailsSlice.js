import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { sendIpcMessage, IPC_TYPES } from '../../utils/ipc';

export const selectAnime = createAsyncThunk(
  'animeDetails/selectAnime',
  async (anime, { rejectWithValue }) => {
    try {
      const response = await sendIpcMessage(IPC_TYPES.ANIME_SELECTED, {
        shikimoriId: anime.id
      });

      if (response.type === 'error') {
        throw new Error(response.data.message);
      }

      return {
        ...anime,
        translations: response.data.translations.map(t => ({
          id: t.id,
          title: t.title,
          episodes: t.episodes
        })),
        episodeCount: response.data.episodes
      };
    } catch (err) {
      return rejectWithValue(err.message);
    }
  }
);

const animeDetailsSlice = createSlice({
  name: 'animeDetails',
  initialState: {
    selectedAnime: null,
    translations: [],
    episodeCount: 0,
    selectedTranslation: null,
    status: 'idle',
    error: null
  },
  reducers: {
    setSelectedTranslation: (state, action) => {
      state.selectedTranslation = action.payload;
      if (action.payload) {
        state.episodeCount = Math.min(
          state.episodeCount,
          action.payload.episodes
        );
      }
    },
    clearSelection: (state) => {
      state.selectedAnime = null;
      state.translations = [];
      state.episodeCount = 0;
      state.selectedTranslation = null;
      state.status = 'idle';
      state.error = null;
    }
  },
  extraReducers: (builder) => {
    builder
      .addCase(selectAnime.pending, (state) => {
        state.status = 'loading';
        state.error = null;
      })
      .addCase(selectAnime.fulfilled, (state, action) => {
        state.status = 'succeeded';
        state.selectedAnime = action.payload;
        state.translations = action.payload.translations;
        state.episodeCount = action.payload.episodeCount;
        if (action.payload.translations?.length > 0) {
          const firstTranslation = action.payload.translations[0];
          state.selectedTranslation = firstTranslation;
          state.episodeCount = Math.min(
            action.payload.episodeCount,
            firstTranslation.episodes
          );
        }
      })
      .addCase(selectAnime.rejected, (state, action) => {
        state.status = 'failed';
        state.error = action.payload;
      });
  }
});

export const { setSelectedTranslation, clearSelection } = animeDetailsSlice.actions;
export default animeDetailsSlice.reducer; 