import { useEffect, useState } from 'preact/hooks';
import { route } from 'preact-router';
import { useSelector } from 'react-redux';
import { Player } from './Player';

export function Sidebar() {
  const [currentPath, setCurrentPath] = useState(window.location.pathname);
  const theme = useSelector(state => state.theme.current);

  useEffect(() => {
    const handleRouteChange = () => {
      setCurrentPath(window.location.pathname);
    };

    // Listen for route changes
    window.addEventListener('popstate', handleRouteChange);
    return () => window.removeEventListener('popstate', handleRouteChange);
  }, []);

  const navItems = [
    { label: 'Home', path: '/', icon: 'home' },
    { label: 'Search', path: '/search', icon: 'search' },
    { label: 'Settings', path: '/settings', icon: 'cog' }
  ];

  const handleNavClick = (e, path) => {
    e.preventDefault();
    route(path);
    setCurrentPath(path); // Update immediately for better UX
  };

  const isActive = (path) => {
    if (path === '/') {
      return currentPath === path;
    }
    return currentPath.startsWith(path);
  };

  return (
    <aside className="w-64 h-screen bg-light-background-alt dark:bg-dark-background-alt border-r border-light-border dark:border-dark-border flex flex-col backdrop-blur-glass transition-colors duration-200">
      <nav className="flex-1 flex flex-col gap-2 p-4">
        {navItems.map(item => (
          <a
            key={item.path}
            href={item.path}
            onClick={(e) => handleNavClick(e, item.path)}
            className={`
              flex items-center gap-3 px-4 py-3 rounded-lg
              transition-all duration-200 ease-in-out
              group relative
              ${isActive(item.path)
                ? `
                  bg-primary text-white
                  shadow-lg shadow-primary/20
                  hover:shadow-xl hover:shadow-primary/30
                  scale-[1.02]
                `
                : `
                  text-light-text-secondary dark:text-dark-text-secondary
                  hover:bg-light-card-hover dark:hover:bg-dark-card-hover
                  hover:text-light-text dark:hover:text-dark-text
                  hover:scale-[1.02]
                `
              }
            `}
          >
            <i className={`
              fas fa-${item.icon}
              transition-all duration-300
              ${isActive(item.path)
                ? 'transform scale-110 animate-slide-up'
                : 'group-hover:scale-110'
              }
            `} />
            <span className={`
              transition-all duration-200
              ${isActive(item.path)
                ? 'font-medium'
                : 'group-hover:font-medium'
              }
            `}>
              {item.label}
            </span>
            
            {isActive(item.path) && (
              <span className="absolute inset-y-0 left-0 w-1 bg-white rounded-full" />
            )}
          </a>
        ))}
      </nav>

      <div className="border-t border-light-border dark:border-dark-border pt-4">
        <Player />
      </div>
    </aside>
  );
} 