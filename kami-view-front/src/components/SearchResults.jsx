import { AnimeGrid } from './AnimeGrid';
import { Loading } from './ui/Loading';
import { useSearch } from '../hooks/useSearch';

export function SearchResults() {
  const { results, isLoading, error } = useSearch();

  if (isLoading) {
    return (
      <div className="flex justify-center py-12">
        <Loading />
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-12">
        <div className="text-error mb-2">
          <i className="fas fa-exclamation-circle text-2xl" />
        </div>
        <p className="text-light-text-secondary dark:text-dark-text-secondary">
          Failed to load results: {error}
        </p>
      </div>
    );
  }

  return (
    <div className="p-6">
      <AnimeGrid items={results} />
    </div>
  );
} 