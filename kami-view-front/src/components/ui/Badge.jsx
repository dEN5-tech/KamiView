const variants = {
  primary: 'bg-primary/10 text-primary border-primary/20',
  success: 'bg-success/10 text-success border-success/20',
  error: 'bg-error/10 text-error border-error/20',
  warning: 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20',
  default: 'bg-light-card dark:bg-dark-card text-light-text dark:text-dark-text border-light-border dark:border-dark-border',
};

export function Badge({ 
  children, 
  variant = 'default',
  icon,
  className = '' 
}) {
  return (
    <span className={`
      inline-flex items-center gap-1
      px-2 py-1 text-xs font-medium
      rounded-full border
      ${variants[variant]}
      ${className}
    `}>
      {icon && <i className={`fas fa-${icon}`} />}
      {children}
    </span>
  );
} 