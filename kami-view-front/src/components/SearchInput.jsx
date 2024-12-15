import { motion } from 'framer-motion';
import { Input } from './ui/Input';
import { Button } from './ui/Button';

export function SearchInput({ 
  value = '',
  onChange, 
  onSubmit,
  loading = false,
  className = '' 
}) {
  return (
    <div className={`relative ${className}`}>
      <Input
        type="text"
        value={value || ''}
        onInput={(e) => onChange(e.target.value || '')}
        onKeyDown={(e) => e.key === 'Enter' && onSubmit?.()}
        placeholder="Search anime..."
        icon={loading ? 'circle-notch fa-spin' : 'search'}
        className="pr-20"
        disabled={loading}
      />
      
      {value && !loading && (
        <motion.div
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.8 }}
          className="absolute right-2 top-1/2 -translate-y-1/2"
        >
          <Button
            variant="secondary"
            size="sm"
            icon="times"
            onClick={() => onChange('')}
            aria-label="Clear search"
          />
        </motion.div>
      )}
    </div>
  );
} 