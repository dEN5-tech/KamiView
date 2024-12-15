import { useEffect, useState } from 'preact/hooks';
import { Router } from 'preact-router';
import { ThemeProvider } from './components/ThemeProvider';
import { Header } from './components/Header';
import { Sidebar } from './components/Sidebar';
import { Home, Search, Settings, AnimeDetails } from './pages';
import { Loading } from './components/ui/Loading';
import { initIpc } from './utils/ipc';

export function App() {
  const [ipcReady, setIpcReady] = useState(false);
  const [ipcError, setIpcError] = useState(null);

  useEffect(() => {
    const init = async () => {
      try {
        await initIpc();
        setIpcReady(true);
      } catch (err) {
        console.error('Failed to initialize IPC:', err);
        setIpcError(err.message);
      }
    };
    init();
  }, []);

  if (ipcError) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-center">
          <h1 className="text-2xl font-bold text-error mb-4">Connection Error</h1>
          <p className="text-light-text-secondary dark:text-dark-text-secondary">
            Failed to connect to the application: {ipcError}
          </p>
        </div>
      </div>
    );
  }

  if (!ipcReady) {
    return (
      <div className="flex items-center justify-center h-screen">
        <Loading />
      </div>
    );
  }

  return (
    <ThemeProvider>
      <div className="flex h-screen bg-light-background dark:bg-dark-background text-light-text dark:text-dark-text">
        <Sidebar />
        <div className="flex flex-col flex-1 overflow-hidden">
          <Header />
          <main className="flex-1 overflow-auto">
            <Router>
              <Home path="/" />
              <Search path="/search" />
              <Settings path="/settings" />
              <AnimeDetails path="/anime/:id" />
            </Router>
          </main>
        </div>
      </div>
    </ThemeProvider>
  );
}
