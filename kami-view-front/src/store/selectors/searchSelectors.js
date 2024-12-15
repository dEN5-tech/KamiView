// Search selectors
export const selectSearchState = state => state.search;
export const selectSearchQuery = state => state.search.query;
export const selectSearchResults = state => state.search.results;
export const selectSearchLoading = state => state.search.loading;
export const selectSearchError = state => state.search.error;
export const selectSearchPage = state => state.search.page;
export const selectSearchHasMore = state => state.search.hasMore; 