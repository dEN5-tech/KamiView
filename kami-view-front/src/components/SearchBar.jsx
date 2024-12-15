import { useEffect } from 'preact/hooks';
import { useDispatch, useSelector } from 'react-redux';
import { Input } from './ui/Input';
import { setQuery, setResults, setLoading, setError, clearSearch } from '../store/slices/searchSlice';
import { sendIpcMessage, IPC_TYPES } from '../utils/ipc';

export function SearchBar() {
  const dispatch = useDispatch();
  const { query, isLoading } = useSelector(state => state.search);

  const handleSearch = async (newQuery) => {
    dispatch(setQuery(newQuery));
    
    if (!newQuery.trim()) {
      dispatch(clearSearch());
      return;
    }

    dispatch(setLoading(true));
    try {
      const results = await sendIpcMessage(IPC_TYPES.SEARCH, { query: newQuery });
      dispatch(setResults(results));
    } catch (err) {
      dispatch(setError(err.message));
    }
  };

  return (
    <Input
      type="text"
      value={query}
      onInput={(e) => handleSearch(e.target.value)}
      placeholder="Search anime..."
      icon={isLoading ? 'circle-notch fa-spin' : 'search'}
      className="w-full"
    />
  );
} 