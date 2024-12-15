import { motion } from 'framer-motion';
import { AnimeCard } from './AnimeCard';
import { staggerContainer } from '../utils/motion';
import { useDispatch } from 'react-redux';
import { selectAnime } from '../store/slices/animeDetailsSlice';

export function AnimeGrid({ items = [] }) {
  const dispatch = useDispatch();

  const handleAnimeSelect = async (anime) => {
    console.log(anime);
    dispatch(selectAnime(anime));
  };

  if (!items?.length) {
    return (
      <div className="text-center py-12">
        <i className="fas fa-film text-4xl text-light-text-muted dark:text-dark-text-muted mb-4" />
        <p className="text-light-text-secondary dark:text-dark-text-secondary">
          No results found
        </p>
      </div>
    );
  }

  return (
    <motion.div
      variants={staggerContainer}
      initial="initial"
      animate="animate"
      className="grid gap-6 grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
    >
      {items.map(anime => (
        <motion.div
          key={anime.id}
          variants={staggerContainer}
          onClick={() => handleAnimeSelect(anime)}
        >
          <AnimeCard anime={anime} />
        </motion.div>
      ))}
    </motion.div>
  );
} 