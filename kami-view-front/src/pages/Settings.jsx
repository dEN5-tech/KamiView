import { useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { motion, AnimatePresence } from 'framer-motion';
import { Card } from '../components/ui/Card';
import { Button } from '../components/ui/Button';
import { Input } from '../components/ui/Input';
import { toggleTheme } from '../store/slices/themeSlice';
import { useAuth } from '../hooks/useAuth';

export function Settings() {
  const dispatch = useDispatch();
  const theme = useSelector(state => state.theme.current);
  const { user, isAuthenticated, login, handleCallback, logout } = useAuth();
  const [authCode, setAuthCode] = useState('');
  const [showCodeInput, setShowCodeInput] = useState(false);

  const handleLogin = async () => {
    await login();
    setShowCodeInput(true); // Show input after opening auth URL
  };

  const handleSubmitCode = async () => {
    if (authCode) {
      await handleCallback(authCode);
      setAuthCode('');
      setShowCodeInput(false);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="container mx-auto px-6 py-8"
    >
      <h1 className="text-2xl font-bold mb-6">Settings</h1>

      <Card className="p-6 mb-6">
        <h2 className="text-lg font-semibold mb-4">Appearance</h2>
        <div className="flex items-center justify-between">
          <span>Theme</span>
          <Button
            onClick={() => dispatch(toggleTheme())}
            variant="secondary"
            icon={theme === 'dark' ? 'moon' : 'sun'}
          >
            {theme === 'dark' ? 'Dark' : 'Light'}
          </Button>
        </div>
      </Card>

      <Card className="p-6">
        <h2 className="text-lg font-semibold mb-4">Shikimori Account</h2>
        
        {!isAuthenticated ? (
          <div className="space-y-4">
            <AnimatePresence mode="wait">
              {showCodeInput ? (
                <motion.div
                  key="code-input"
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -20 }}
                  className="space-y-4"
                >
                  <Input
                    label="Authorization Code"
                    value={authCode}
                    onChange={(e) => setAuthCode(e.target.value)}
                    placeholder="Enter the code from Shikimori"
                    icon="key"
                  />
                  <div className="flex gap-2">
                    <Button onClick={handleSubmitCode} variant="primary">
                      Submit Code
                    </Button>
                    <Button 
                      onClick={() => setShowCodeInput(false)} 
                      variant="secondary"
                    >
                      Cancel
                    </Button>
                  </div>
                </motion.div>
              ) : (
                <motion.div
                  key="connect-button"
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -20 }}
                >
                  <Button onClick={handleLogin} variant="primary" icon="link">
                    Connect Shikimori Account
                  </Button>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        ) : (
          <motion.div 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-4"
          >
            <div className="flex items-center space-x-4">
              <img 
                src={user.avatar} 
                alt={user.username} 
                className="w-12 h-12 rounded-full"
              />
              <div>
                <h3 className="font-medium">{user.username}</h3>
                <p className="text-sm text-gray-500">Connected to Shikimori</p>
              </div>
            </div>
            <Button onClick={logout} variant="secondary" icon="unlink">
              Disconnect Account
            </Button>
          </motion.div>
        )}
      </Card>
    </motion.div>
  );
} 