import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { sendIpcMessage, IPC_TYPES } from '../../utils/ipc';

const initialState = {
  user: null,
  status: 'idle',
  error: null
};

export const startAuth = createAsyncThunk(
  'auth/startAuth',
  async () => {
    const response = await sendIpcMessage(IPC_TYPES.OPEN_AUTH_URL);
    return response;
  }
);

export const handleAuth = createAsyncThunk(
  'auth/handleAuth',
  async (code) => {
    const response = await sendIpcMessage(IPC_TYPES.EXCHANGE_CODE, { code });
    return response.data.data;
  }
);

export const logoutUser = createAsyncThunk(
  'auth/logout',
  async () => {
    await sendIpcMessage(IPC_TYPES.LOGOUT);
  }
);

const authSlice = createSlice({
  name: 'auth',
  initialState,
  reducers: {
    clearError: (state) => {
      state.error = null;
    }
  },
  extraReducers: (builder) => {
    builder
      // Start Auth
      .addCase(startAuth.pending, (state) => {
        state.status = 'loading';
      })
      .addCase(startAuth.fulfilled, (state) => {
        state.status = 'idle';
      })
      .addCase(startAuth.rejected, (state, action) => {
        state.status = 'failed';
        state.error = action.error.message;
      })
      // Handle Auth
      .addCase(handleAuth.pending, (state) => {
        state.status = 'loading';
      })
      .addCase(handleAuth.fulfilled, (state, action) => {
        state.status = 'succeeded';
        state.user = {
          username: action.payload.username,
          avatar: action.payload.avatar,
          id: action.payload.id
        };
      })
      .addCase(handleAuth.rejected, (state, action) => {
        state.status = 'failed';
        state.error = action.error.message;
      })
      // Logout
      .addCase(logoutUser.fulfilled, (state) => {
        state.user = null;
        state.status = 'idle';
      });
  }
});

export const { clearError } = authSlice.actions;
export default authSlice.reducer; 