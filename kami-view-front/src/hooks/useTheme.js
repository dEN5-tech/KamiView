import { useSelector, useDispatch } from 'react-redux';
import { setTheme, toggleTheme } from '../store/slices/themeSlice';

export function useTheme() {
  const dispatch = useDispatch();
  const theme = useSelector(state => state.theme.current);

  const handleSetTheme = (newTheme) => {
    dispatch(setTheme(newTheme));
  };

  const handleToggleTheme = () => {
    dispatch(toggleTheme());
  };

  return {
    theme,
    setTheme: handleSetTheme,
    toggleTheme: handleToggleTheme
  };
} 