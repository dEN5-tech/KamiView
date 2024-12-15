import { useEffect } from 'preact/hooks';
import { useSelector, useDispatch } from 'react-redux';
import { searchAnime, setQuery, clearSearch } from '../store/slices/searchSlice';
import { useDebounce } from './useDebounce';

export function useSearch() {
  const dispatch = useDispatch();
  const { query, results, status, error } = useSelector(state => state.search);
  const debouncedQuery = useDebounce(query, 500);

  useEffect(() => {
    if (debouncedQuery?.trim()) {
      dispatch(searchAnime(debouncedQuery));
    } else {
      dispatch(clearSearch());
    }
  }, [debouncedQuery, dispatch]);

  const handleSearch = (value) => {
    dispatch(setQuery(value));
  };

  return {
    query,
    results,
    isLoading: status === 'loading',
    error,
    handleSearch
  };
} 