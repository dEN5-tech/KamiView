import { forwardRef } from 'preact/compat';
import { motion, AnimatePresence } from 'framer-motion';

export const Input = forwardRef(({ 
  label,
  error,
  icon,
  className = '',
  ...props 
}, ref) => {
  return (
    <div className="w-full">
      <AnimatePresence>
        {label && (
          <motion.label
            initial={{ y: -10, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            exit={{ y: -10, opacity: 0 }}
            className="block text-sm font-medium text-light-text-secondary dark:text-dark-text-secondary mb-1"
          >
            {label}
          </motion.label>
        )}
      </AnimatePresence>
      
      <div className="relative">
        {icon && (
          <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <i className={`fas fa-${icon} text-light-text-muted dark:text-dark-text-muted`} />
          </div>
        )}
        
        <input
          ref={ref}
          className={`
            w-full px-4 py-2 ${icon ? 'pl-10' : ''}
            bg-light-card dark:bg-dark-card
            border border-light-border dark:border-dark-border
            text-light-text dark:text-dark-text
            placeholder-light-text-muted dark:placeholder-dark-text-muted
            rounded-lg
            focus:ring-2 focus:ring-primary/50 focus:border-primary
            transition-all duration-200
            disabled:opacity-50 disabled:cursor-not-allowed
            hover:border-primary/50
            ${error ? 'border-error focus:ring-error/50 focus:border-error' : ''}
            ${className}
          `}
          {...props}
        />
        
        <AnimatePresence>
          {error && (
            <motion.div
              initial={{ opacity: 0, y: -10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="mt-1 text-sm text-error flex items-center gap-1"
            >
              <i className="fas fa-exclamation-circle" />
              {error}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}); 