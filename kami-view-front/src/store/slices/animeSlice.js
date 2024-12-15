import { createSlice } from '@reduxjs/toolkit';

const initialState = {
  searchQuery: '',
  searchResults: [],
  selectedAnime: null,
  animeDetails: null,
  isLoading: false,
  error: null,
  playbackInfo: null,
  currentEpisode: null,
  progress: { percent: 0, current: 0, total: 0 },
  isGenerating: false,
  selectedTranslation: null,
  isTranslationLoading: false,
};

export const animeSlice = createSlice({
  name: 'anime',
  initialState,
  reducers: {
    setSearchQuery: (state, action) => {
      state.searchQuery = action.payload;
    },
    setSearchResults: (state, action) => {
      state.searchResults = action.payload;
      state.isLoading = false;
    },
    setSelectedAnime: (state, action) => {
      state.selectedAnime = action.payload;
    },
    setAnimeDetails: (state, action) => {
      state.animeDetails = action.payload;
      state.isLoading = false;
    },
    setLoading: (state, action) => {
      state.isLoading = action.payload;
    },
    setError: (state, action) => {
      state.error = action.payload;
    },
    setPlaybackInfo: (state, action) => {
      state.playbackInfo = action.payload.info;
      state.currentEpisode = action.payload.currentEpisode;
    },
    setProgress: (state, action) => {
      state.progress = action.payload;
    },
    setGenerating: (state, action) => {
      state.isGenerating = action.payload;
    },
    setSelectedTranslation: (state, action) => {
      state.selectedTranslation = action.payload;
    },
    setTranslationLoading: (state, action) => {
      state.isTranslationLoading = action.payload;
    },
  },
});

export const {
  setSearchQuery,
  setSearchResults,
  setSelectedAnime,
  setAnimeDetails,
  setLoading,
  setError,
  setPlaybackInfo,
  setProgress,
  setGenerating,
  setSelectedTranslation,
  setTranslationLoading,
} = animeSlice.actions;

export default animeSlice.reducer; 