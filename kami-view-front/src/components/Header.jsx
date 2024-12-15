import { useTheme } from '../hooks/useTheme';
import { useAuth } from '../hooks/useAuth';
import { route } from 'preact-router';

export function Header() {
  const { theme, toggleTheme } = useTheme();
  const { user, isAuthenticated } = useAuth();

  const handleProfileClick = (e) => {
    e.preventDefault();
    route('/settings');
  };

  return (
    <header className="flex items-center justify-between h-16 px-6 bg-card border-b border-white/10">
      <h1 className="text-xl font-bold">KamiView</h1>
      
      <div className="flex items-center gap-4">
        {isAuthenticated && user ? (
          <a 
            href="/settings"
            onClick={handleProfileClick}
            className="flex items-center gap-2 hover:opacity-80 transition-opacity"
          >
            {user.avatar && (
              <img 
                src={user.avatar}
                alt={user.username}
                className="w-8 h-8 rounded-full object-cover bg-light-card-hover dark:bg-dark-card-hover"
              />
            )}
            <span className="text-sm font-medium hidden sm:block">
              {user.username}
            </span>
          </a>
        ) : (
          <button
            onClick={toggleTheme}
            className="p-2 rounded-lg hover:bg-light-card-hover dark:hover:bg-dark-card-hover"
          >
            <i className={`fas fa-${theme === 'light' ? 'moon' : 'sun'}`} />
          </button>
        )}
      </div>
    </header>
  );
} 