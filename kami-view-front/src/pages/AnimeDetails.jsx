import { useEffect } from 'preact/hooks';
import { motion, AnimatePresence } from 'framer-motion';
import { useAnimeDetails } from '../hooks/useAnimeDetails';
import { usePlayback } from '../hooks/usePlayback';
import { Loading } from '../components/ui/Loading';
import { Card } from '../components/ui/Card';
import { Badge } from '../components/ui/Badge';

export function AnimeDetails({ id }) {
  const { 
    selectedAnime,
    translations,
    episodeCount,
    selectedTranslation,
    isLoading,
    error,
    selectTranslation 
  } = useAnimeDetails();

  const { playEpisode } = usePlayback();

  const handlePlay = (episodeNumber) => {
    if (!selectedTranslation) return;
    playEpisode(id, episodeNumber, selectedTranslation.id);
  };

  const badgeVariants = {
    selected: {
      scale: 1.05,
      backgroundColor: 'var(--color-primary)',
      color: 'var(--color-primary-foreground)',
      transition: {
        type: 'spring',
        stiffness: 500,
        damping: 30
      }
    },
    unselected: {
      scale: 1,
      backgroundColor: 'var(--color-card)',
      color: 'var(--color-text)',
      transition: {
        type: 'spring',
        stiffness: 500,
        damping: 30
      }
    }
  };

  const handleTranslationSelect = (translation) => {
    selectTranslation(translation);
  };

  if (isLoading) {
    return (
      <div className="flex justify-center items-center h-full">
        <Loading />
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex justify-center items-center h-full text-error">
        <div className="text-center">
          <i className="fas fa-exclamation-circle text-3xl mb-2" />
          <p>{error}</p>
        </div>
      </div>
    );
  }

  if (!selectedAnime) return null;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="container mx-auto px-6 py-8"
    >
      <Card className="p-6">
        <div className="flex flex-col gap-6">
          {/* Anime Info */}
          <div className="flex gap-6">
            <img 
              src={selectedAnime.image} 
              alt={selectedAnime.title}
              className="w-48 h-auto rounded-lg shadow-lg"
            />
            <div>
              <h1 className="text-2xl font-bold mb-2">{selectedAnime.title}</h1>
              <div className="flex gap-2 mb-2">
                <Badge>{selectedAnime.year}</Badge>
                {selectedAnime.rating && (
                  <Badge variant="primary">★ {selectedAnime.rating}</Badge>
                )}
              </div>
              <p className="text-light-text-secondary dark:text-dark-text-secondary">
                {selectedAnime.description}
              </p>
            </div>
          </div>

          {/* Updated Translations Selection */}
          {translations?.length > 0 && (
            <div className="flex flex-col gap-2">
              <h2 className="text-xl font-semibold">Translations</h2>
              <div className="flex flex-wrap gap-2">
                <AnimatePresence mode="wait">
                  {translations.map(translation => (
                    <motion.div
                      key={translation.id}
                      initial="unselected"
                      animate={selectedTranslation?.id === translation.id ? "selected" : "unselected"}
                      variants={badgeVariants}
                      whileHover={{ scale: 1.05 }}
                      whileTap={{ scale: 0.95 }}
                      onClick={() => handleTranslationSelect(translation)}
                      className="cursor-pointer"
                    >
                      <Badge
                        variant={selectedTranslation?.id === translation.id ? 'primary' : 'default'}
                        className="transition-colors duration-200"
                      >
                        <span className="flex items-center gap-2">
                          {translation.title}
                          <span className="px-1.5 py-0.5 text-xs rounded-full bg-light-card-hover dark:bg-dark-card-hover">
                            {translation.episodes} ep.
                          </span>
                          {selectedTranslation?.id === translation.id && (
                            <motion.i
                              initial={{ scale: 0 }}
                              animate={{ scale: 1 }}
                              className="fas fa-check text-xs"
                            />
                          )}
                        </span>
                      </Badge>
                    </motion.div>
                  ))}
                </AnimatePresence>
              </div>
            </div>
          )}

          {/* Episodes Grid */}
          {selectedTranslation && episodeCount > 0 && (
            <div>
              <h2 className="text-xl font-semibold mb-4">
                Episodes ({selectedTranslation.title})
              </h2>
              <div className="grid gap-4 grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6">
                {Array.from({ length: episodeCount }, (_, i) => i + 1).map(episodeNumber => (
                  <Card 
                    key={episodeNumber} 
                    className="p-4 hover:bg-light-card-hover dark:hover:bg-dark-card-hover cursor-pointer transition-colors"
                    onClick={() => handlePlay(episodeNumber)}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-medium">Episode {episodeNumber}</span>
                      <i className="fas fa-play text-primary" />
                    </div>
                  </Card>
                ))}
              </div>
            </div>
          )}
        </div>
      </Card>
    </motion.div>
  );
} 