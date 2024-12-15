import { configureStore } from '@reduxjs/toolkit';
import themeReducer from './slices/themeSlice';
import searchReducer from './slices/searchSlice';
import playbackReducer from './slices/playbackSlice';
import animeDetailsReducer from './slices/animeDetailsSlice';
import authReducer from './slices/authSlice';

export const store = configureStore({
  reducer: {
    theme: themeReducer,
    search: searchReducer,
    playback: playbackReducer,
    animeDetails: animeDetailsReducer,
    auth: authReducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: false,
    }),
}); 