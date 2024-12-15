import { SearchInput } from '../components/SearchInput';
import { SearchResults } from '../components/SearchResults';
import { useSearch } from '../hooks/useSearch';

export function Search() {
  const { query, handleSearch } = useSearch();

  return (
    <div className="container mx-auto px-4 py-6">
      <div className="max-w-2xl mx-auto mb-8">
        <SearchInput
          value={query}
          onChange={handleSearch}
          className="w-full"
        />
      </div>
      
      {query && (
        <h2 className="text-xl font-semibold mb-6">
          Search results for "{query}"
        </h2>
      )}
      
      <SearchResults />
    </div>
  );
} 