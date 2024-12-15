import { useEffect } from 'preact/hooks';
import { useSelector, useDispatch } from 'react-redux';
import { 
  playEpisode, 
  getPlaybackInfo,
  togglePlayback,
  stopPlayback,
  setCurrentEpisode 
} from '../store/slices/playbackSlice';
import { 
  selectPlaybackInfo, 
  selectCurrentEpisode, 
  selectPlaybackProgress 
} from '../store/selectors/playbackSelectors';

export function usePlayback() {
  const dispatch = useDispatch();
  const playbackInfo = useSelector(selectPlaybackInfo);
  const currentEpisode = useSelector(selectCurrentEpisode);
  const progress = useSelector(selectPlaybackProgress);
  const isLoading = useSelector(state => state.playback.isLoading);

  useEffect(() => {
    if (currentEpisode) {
      const interval = setInterval(() => {
        dispatch(getPlaybackInfo());
      }, 1000);

      return () => clearInterval(interval);
    }
  }, [dispatch, currentEpisode]);

  const handlePlay = async (shikimoriId, episode, translationId) => {
    try {
      await dispatch(playEpisode({ shikimoriId, episode, translationId })).unwrap();
      dispatch(setCurrentEpisode({ shikimoriId, episode, translationId }));
    } catch (err) {
      console.error('Failed to play episode:', err);
    }
  };

  const handleTogglePlayback = () => {
    dispatch(togglePlayback());
  };

  const handleStop = () => {
    dispatch(stopPlayback());
  };

  return {
    playbackInfo,
    currentEpisode,
    progress,
    isLoading,
    playEpisode: handlePlay,
    togglePlayback: handleTogglePlayback,
    stopPlayback: handleStop
  };
} 