import { createAsyncThunk } from '@reduxjs/toolkit';
import { IPC_TYPES, sendIpcMessage } from '../../utils/ipc';
import { 
  setLoading, 
  setError, 
  setResults 
} from '../slices/searchSlice';

export const searchAnime = createAsyncThunk(
  'search/searchAnime',
  async (query, { dispatch }) => {
    try {
      dispatch(setLoading(true));
      const results = await sendIpcMessage(IPC_TYPES.SEARCH, { query });
      dispatch(setResults(results));
      return results;
    } catch (error) {
      dispatch(setError(error.message));
      throw error;
    }
  }
);

export const getAnimeDetails = createAsyncThunk(
  'animeDetails/getAnime',
  async (id) => {
    return await sendIpcMessage(IPC_TYPES.GET_ANIME, { id });
  }
);

export const getEpisodes = createAsyncThunk(
  'animeDetails/getEpisodes',
  async (animeId) => {
    return await sendIpcMessage(IPC_TYPES.GET_EPISODES, { animeId });
  }
);

export const getStreamUrl = createAsyncThunk(
  'playback/getStream',
  async ({ episodeId, quality }) => {
    return await sendIpcMessage(IPC_TYPES.GET_STREAM, { 
      episodeId, 
      quality 
    });
  }
);

export const downloadEpisode = createAsyncThunk(
  'animeDetails/download',
  async ({ episodeId, filename }) => {
    return await sendIpcMessage(IPC_TYPES.DOWNLOAD, {
      episodeId,
      filename
    });
  }
); 