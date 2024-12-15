import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { sendIpcMessage, IPC_TYPES } from '../../utils/ipc';

export const playEpisode = createAsyncThunk(
  'playback/playEpisode',
  async ({ shikimoriId, episode, translationId }, { rejectWithValue }) => {
    try {
      const response = await sendIpcMessage(IPC_TYPES.PLAY_EPISODE, {
        shikimoriId,
        episode,
        translationId
      });

      if (response.type === 'error') {
        throw new Error(response.data.message);
      }

      return response.data;
    } catch (err) {
      return rejectWithValue(err.message);
    }
  }
);

export const getPlaybackInfo = createAsyncThunk(
  'playback/getPlaybackInfo',
  async (_, { rejectWithValue }) => {
    try {
      const response = await sendIpcMessage(IPC_TYPES.GET_PLAYBACK_INFO);
      
      if (response.type === 'error') {
        throw new Error(response.data.message);
      }

      return response.data.data;
    } catch (err) {
      return rejectWithValue(err.message);
    }
  }
);

export const togglePlayback = createAsyncThunk(
  'playback/togglePlayback',
  async (_, { getState, rejectWithValue }) => {
    try {
      const { playbackInfo } = getState().playback;
      const response = await sendIpcMessage(IPC_TYPES.TOGGLE_PLAYBACK, {
        paused: !playbackInfo.paused
      });
      return response.data.data;
    } catch (err) {
      return rejectWithValue(err.message);
    }
  }
);

export const stopPlayback = createAsyncThunk(
  'playback/stopPlayback',
  async (_, { rejectWithValue }) => {
    try {
      const response = await sendIpcMessage(IPC_TYPES.STOP_PLAYBACK);
      return response.data.data;
    } catch (err) {
      return rejectWithValue(err.message);
    }
  }
);

const playbackSlice = createSlice({
  name: 'playback',
  initialState: {
    playbackInfo: {
      position: 0,
      duration: 0,
      paused: true
    },
    currentEpisode: null,
    status: 'idle',
    error: null,
    isLoading: false
  },
  reducers: {
    setCurrentEpisode: (state, action) => {
      state.currentEpisode = action.payload;
    },
    clearPlayback: (state) => {
      state.playbackInfo = {
        position: 0,
        duration: 0,
        paused: true
      };
      state.currentEpisode = null;
      state.status = 'idle';
      state.error = null;
    }
  },
  extraReducers: (builder) => {
    builder
      .addCase(playEpisode.pending, (state) => {
        state.status = 'loading';
        state.error = null;
      })
      .addCase(playEpisode.fulfilled, (state, action) => {
        state.status = 'succeeded';
        state.error = null;
      })
      .addCase(playEpisode.rejected, (state, action) => {
        state.status = 'failed';
        state.error = action.payload;
      })
      .addCase(getPlaybackInfo.fulfilled, (state, action) => {
        state.playbackInfo = action.payload;
        state.error = null;
      })
      .addCase(getPlaybackInfo.rejected, (state, action) => {
        state.error = action.payload;
      })
      .addCase(togglePlayback.pending, (state) => {
        state.isLoading = true;
      })
      .addCase(togglePlayback.fulfilled, (state, action) => {
        state.isLoading = false;
        state.playbackInfo = action.payload;
        state.error = null;
      })
      .addCase(togglePlayback.rejected, (state, action) => {
        state.isLoading = false;
        state.error = action.payload;
      })
      .addCase(stopPlayback.pending, (state) => {
        state.isLoading = true;
      })
      .addCase(stopPlayback.fulfilled, (state) => {
        state.isLoading = false;
        state.currentEpisode = null;
        state.playbackInfo = {
          position: 0,
          duration: 0,
          paused: true
        };
        state.error = null;
      })
      .addCase(stopPlayback.rejected, (state, action) => {
        state.isLoading = false;
        state.error = action.payload;
      });
  }
});

export const { setCurrentEpisode, clearPlayback } = playbackSlice.actions;
export default playbackSlice.reducer; 