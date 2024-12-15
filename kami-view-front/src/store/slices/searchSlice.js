import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { sendIpcMessage, IPC_TYPES } from '../../utils/ipc';

// Async thunk for search
export const searchAnime = createAsyncThunk(
  'search/searchAnime',
  async (query, { rejectWithValue }) => {
    try {
      const response = await sendIpcMessage(IPC_TYPES.SEARCH, { query });
      
      if (!response.data?.results) {
        throw new Error('Invalid response format');
      }

      return response.data.results.map(item => ({
        id: item.shikimori_id || item.kinopoisk_id || item.imdb_id,
        title: item.material_data.title,
        image: item.material_data.poster_url,
        rating: item.material_data.imdb_rating || item.material_data.kinopoisk_rating,
        year: item.material_data.year,
        description: item.material_data.description
      }));
    } catch (err) {
      return rejectWithValue(err.message);
    }
  }
);

const searchSlice = createSlice({
  name: 'search',
  initialState: {
    query: '',
    results: [],
    status: 'idle', // 'idle' | 'loading' | 'succeeded' | 'failed'
    error: null
  },
  reducers: {
    setQuery: (state, action) => {
      state.query = action.payload;
    },
    clearSearch: (state) => {
      state.results = [];
      state.status = 'idle';
      state.error = null;
    }
  },
  extraReducers: (builder) => {
    builder
      .addCase(searchAnime.pending, (state) => {
        state.status = 'loading';
        state.error = null;
      })
      .addCase(searchAnime.fulfilled, (state, action) => {
        state.status = 'succeeded';
        state.results = action.payload;
      })
      .addCase(searchAnime.rejected, (state, action) => {
        state.status = 'failed';
        state.error = action.payload;
      });
  }
});

export const { setQuery, clearSearch } = searchSlice.actions;
export default searchSlice.reducer; 