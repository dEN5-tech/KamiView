// Anime details selectors
export const selectAnimeDetailsState = state => state.animeDetails;
export const selectAnimeDetails = state => state.animeDetails.details;
export const selectSelectedTranslation = state => state.animeDetails.selectedTranslation;
export const selectAnimeLoading = state => state.animeDetails.loading;
export const selectTranslationLoading = state => state.animeDetails.translationLoading;
export const selectAnimeError = state => state.animeDetails.error;
export const selectIsGeneratingPlaylist = state => state.animeDetails.isGeneratingPlaylist;
export const selectShowDownloadModal = state => state.animeDetails.showDownloadModal; 