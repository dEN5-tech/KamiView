import { forwardRef } from 'preact/compat';

const variants = {
  primary: 'bg-primary hover:bg-primary-hover text-white',
  secondary: 'bg-light-card dark:bg-dark-card hover:bg-light-card-hover dark:hover:bg-dark-card-hover text-light-text dark:text-dark-text',
  outline: 'border border-light-border dark:border-dark-border hover:bg-light-card dark:hover:bg-dark-card text-light-text dark:text-dark-text',
  danger: 'bg-error hover:bg-error-hover text-white',
  success: 'bg-success hover:bg-success-hover text-white',
};

const sizes = {
  sm: 'px-3 py-1.5 text-sm',
  md: 'px-4 py-2',
  lg: 'px-6 py-3 text-lg',
};

export const Button = forwardRef(({
  variant = 'primary',
  size = 'md',
  icon,
  children,
  className = '',
  loading = false,
  ...props
}, ref) => {
  return (
    <button
      ref={ref}
      className={`
        inline-flex items-center justify-center gap-2
        font-medium rounded-lg
        transition-all duration-200
        disabled:opacity-50 disabled:cursor-not-allowed
        shadow-lg shadow-primary/20 hover:shadow-xl
        ${variants[variant]}
        ${sizes[size]}
        ${className}
      `}
      disabled={loading || props.disabled}
      {...props}
    >
      {loading ? (
        <i className="fas fa-circle-notch fa-spin" />
      ) : icon && (
        <i className={`fas fa-${icon}`} />
      )}
      {children}
    </button>
  );
}); 