import { motion } from 'framer-motion';
import { scaleIn } from '../../utils/motion';

export function Card({ 
  children, 
  hover = false,
  animate = true,
  className = '',
  onClick,
}) {
  const Component = animate ? motion.div : 'div';
  
  return (
    <Component
      variants={animate ? scaleIn : undefined}
      initial="initial"
      animate="animate"
      exit="exit"
      onClick={onClick}
      className={`
        bg-light-card dark:bg-dark-card
        backdrop-blur-glass
        border border-light-border dark:border-dark-border
        rounded-xl
        ${hover ? `
          shadow-lg hover:shadow-xl
          hover:scale-[1.02]
          cursor-pointer
        ` : ''}
        transition-all duration-300 ease-out
        ${className}
      `}
    >
      {children}
    </Component>
  );
} 