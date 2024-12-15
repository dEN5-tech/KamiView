import { useDispatch, useSelector } from 'react-redux';
import { startAuth, handleAuth, logoutUser, clearError } from '../store/slices/authSlice';

export function useAuth() {
  const dispatch = useDispatch();
  const { user, status, error } = useSelector(state => state.auth);

  const login = async () => {
    try {
      await dispatch(startAuth()).unwrap();
    } catch (err) {
      console.error('Login failed:', err);
    }
  };

  const handleCallback = async (code) => {
    try {
      await dispatch(handleAuth(code)).unwrap();
    } catch (err) {
      console.error('Auth callback failed:', err);
    }
  };

  const logout = async () => {
    try {
      await dispatch(logoutUser()).unwrap();
    } catch (err) {
      console.error('Logout failed:', err);
    }
  };

  return {
    user,
    isAuthenticated: !!user,
    isLoading: status === 'loading',
    error,
    login,
    logout,
    handleCallback,
    clearError: () => dispatch(clearError())
  };
} 