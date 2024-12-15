import { usePlayback } from '../hooks/usePlayback';
import { motion, AnimatePresence } from 'framer-motion';
import { Card } from './ui/Card';
import { useEffect, useState } from 'preact/hooks';

export function Player() {
  const { playbackInfo, currentEpisode, progress, isLoading, togglePlayback, stopPlayback } = usePlayback();
  const [shouldShow, setShouldShow] = useState(true);

  useEffect(() => {
    if (playbackInfo && Object.keys(playbackInfo).length === 0) {
      // Start timeout when playbackInfo is empty
      const timeoutId = setTimeout(() => {
        setShouldShow(false);
        stopPlayback(); // Clean up playback state
      }, 15000); // 15 seconds

      return () => {
        clearTimeout(timeoutId);
      };
    } else {
      setShouldShow(true);
    }
  }, [playbackInfo, stopPlayback]);

  // Don't render if no episode, loading, or shouldn't show
  if (!currentEpisode || isLoading || !shouldShow) {
    return null;
  }

  const formatTime = (seconds) => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = Math.floor(seconds % 60);
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
  };

  return (
    <AnimatePresence>
      <motion.div
        initial={{ y: 100, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        exit={{ y: 100, opacity: 0 }}
        className="fixed bottom-0 left-0 right-0 z-50"
      >
        <Card className="p-4 rounded-none border-t border-light-border dark:border-dark-border">
          <div className="flex flex-col gap-2 max-w-7xl mx-auto">
            {/* Episode Info */}
            <div className="flex items-center justify-between text-sm">
              <span className="font-medium">
                Episode {currentEpisode.episode} • {currentEpisode.translation}
              </span>
              <span className="text-light-text-secondary dark:text-dark-text-secondary">
                {formatTime(playbackInfo.position || 0)} / {formatTime(playbackInfo.duration || 0)}
              </span>
            </div>

            {/* Progress Bar */}
            <div className="relative h-1 bg-light-border dark:bg-dark-border rounded-full overflow-hidden">
              <motion.div 
                className="absolute inset-y-0 left-0 bg-primary"
                initial={{ width: 0 }}
                animate={{ 
                  width: `${((playbackInfo.position || 0) / (playbackInfo.duration || 1)) * 100}%` 
                }}
                transition={{ type: "spring", bounce: 0 }}
              />
            </div>

            {/* Controls */}
            <div className="flex items-center justify-center gap-4">
              <button
                onClick={togglePlayback}
                className="p-2 rounded-full hover:bg-light-card-hover dark:hover:bg-dark-card-hover transition-colors"
              >
                <i className={`fas fa-${playbackInfo.paused ? 'play' : 'pause'} text-lg`} />
              </button>
              <button
                onClick={() => {
                  stopPlayback();
                  setShouldShow(false);
                }}
                className="p-2 rounded-full hover:bg-light-card-hover dark:hover:bg-dark-card-hover transition-colors"
              >
                <i className="fas fa-stop text-lg" />
              </button>
            </div>
          </div>
        </Card>
      </motion.div>
    </AnimatePresence>
  );
} 