import { useState } from 'preact/hooks';

export function Tooltip({ 
  children, 
  content,
  position = 'top'
}) {
  const [show, setShow] = useState(false);

  const positions = {
    top: 'bottom-full mb-2',
    bottom: 'top-full mt-2',
    left: 'right-full mr-2',
    right: 'left-full ml-2',
  };

  return (
    <div className="relative inline-block">
      <div
        onMouseEnter={() => setShow(true)}
        onMouseLeave={() => setShow(false)}
      >
        {children}
      </div>

      {show && (
        <div className={`
          absolute z-50 ${positions[position]} 
          px-2 py-1 text-sm
          bg-light-background-alt dark:bg-dark-background-alt
          text-light-text dark:text-dark-text
          rounded shadow-lg
          whitespace-nowrap
          animate-fade-in
        `}>
          {content}
        </div>
      )}
    </div>
  );
} 