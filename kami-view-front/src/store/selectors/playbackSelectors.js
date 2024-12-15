// Playback selectors
export const selectPlaybackState = state => state.playback;
export const selectPlaybackInfo = state => state.playback.playbackInfo;
export const selectCurrentEpisode = state => state.playback.currentEpisode;
export const selectPlaybackProgress = state => {
  const { position, duration } = state.playback.playbackInfo;
  return duration > 0 ? (position / duration) * 100 : 0;
};
export const selectIsPlaying = (state) => state.playback.isPlaying;
export const selectIsGenerating = state => state.playback.isGenerating; 