import { useSelector, useDispatch } from 'react-redux';
import { selectAnime, setSelectedTranslation } from '../store/slices/animeDetailsSlice';

export function useAnimeDetails() {
  const dispatch = useDispatch();
  const {
    selectedAnime,
    translations,
    episodeCount,
    selectedTranslation,
    status,
    error
  } = useSelector(state => state.animeDetails);

  const handleSelectAnime = async (anime) => {
    try {
      await dispatch(selectAnime(anime)).unwrap();
    } catch (err) {
      console.error('Failed to select anime:', err);
    }
  };

  const handleSelectTranslation = (translation) => {
    dispatch(setSelectedTranslation(translation));
  };

  return {
    selectedAnime,
    translations,
    episodeCount,
    selectedTranslation,
    isLoading: status === 'loading',
    error,
    selectAnime: handleSelectAnime,
    selectTranslation: handleSelectTranslation
  };
} 