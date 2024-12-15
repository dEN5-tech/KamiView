export function Loading({ size = 'md' }) {
  const sizes = {
    sm: 'w-4 h-4',
    md: 'w-8 h-8',
    lg: 'w-12 h-12',
  };

  return (
    <div className="flex items-center justify-center p-4">
      <div className={`
        ${sizes[size]}
        border-4 border-light-border dark:border-dark-border
        border-t-primary
        rounded-full
        animate-spin
      `} />
    </div>
  );
} 