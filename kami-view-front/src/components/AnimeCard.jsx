import { useState } from 'preact/hooks';
import { route } from 'preact-router';
import { motion, AnimatePresence } from 'framer-motion';
import { Card } from './ui/Card';
import { Loading } from './ui/Loading';
import { Badge } from './ui/Badge';

export function AnimeCard({ anime }) {
  const [imageLoaded, setImageLoaded] = useState(false);
  const [isHovered, setIsHovered] = useState(false);

  const handleClick = () => {
    route(`/anime/${anime.id}`);
  };

  return (
    <Card 
      hover 
      onClick={handleClick}
      className="group"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <div className="aspect-[2/3] relative overflow-hidden rounded-xl">
        <AnimatePresence>
          {!imageLoaded && (
            <motion.div
              initial={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="absolute inset-0 flex items-center justify-center bg-light-card dark:bg-dark-card"
            >
              <Loading size="sm" />
            </motion.div>
          )}
        </AnimatePresence>

        <motion.img
          src={anime.image}
          alt={anime.title}
          animate={{ 
            scale: isHovered ? 1.1 : 1,
            filter: isHovered ? 'brightness(0.7)' : 'brightness(1)'
          }}
          transition={{ duration: 0.3 }}
          className={`
            w-full h-full object-cover
            transition-opacity duration-300
            ${imageLoaded ? 'opacity-100' : 'opacity-0'}
          `}
          onLoad={() => setImageLoaded(true)}
          loading="lazy"
        />

        <motion.div
          animate={{ opacity: isHovered ? 1 : 0 }}
          className="absolute inset-0 bg-gradient-to-t from-black/80 via-black/40 to-transparent"
        >
          <div className="absolute bottom-0 p-4 w-full">
            <motion.h3
              initial={{ y: 20, opacity: 0 }}
              animate={{ y: isHovered ? 0 : 20, opacity: isHovered ? 1 : 0 }}
              className="text-lg font-semibold text-white line-clamp-2 mb-2"
            >
              {anime.title}
            </motion.h3>

            <motion.div
              initial={{ y: 20, opacity: 0 }}
              animate={{ y: isHovered ? 0 : 20, opacity: isHovered ? 1 : 0 }}
              transition={{ delay: 0.1 }}
              className="flex flex-wrap gap-2"
            >
              {anime.rating && (
                <Badge variant="primary" icon="star">
                  {anime.rating}
                </Badge>
              )}
              {anime.year && (
                <Badge variant="default" icon="calendar">
                  {anime.year}
                </Badge>
              )}
            </motion.div>
          </div>
        </motion.div>
      </div>
    </Card>
  );
} 