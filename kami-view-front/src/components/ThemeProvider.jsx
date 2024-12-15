import { useEffect } from 'preact/hooks';
import { useTheme } from '../hooks/useTheme';

export function ThemeProvider({ children }) {
  const { theme } = useTheme();

  useEffect(() => {
    const root = document.documentElement;
    root.classList.remove('light', 'dark');
    root.classList.add(theme);
    
    // Add transition class after theme is set
    setTimeout(() => {
      root.classList.add('theme-transition');
    }, 0);

    return () => {
      root.classList.remove('theme-transition');
    };
  }, [theme]);

  return (
    <div className="transition-colors duration-200 ease-in-out">
      {children}
    </div>
  );
} 