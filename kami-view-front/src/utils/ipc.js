// IPC message types
export const IPC_TYPES = {
  SEARCH: 'search',
  ANIME_SELECTED: 'animeSelected',
  PLAY_EPISODE: 'playEpisode',
  GET_PLAYBACK_INFO: 'getPlaybackInfo',
  TOGGLE_PLAYBACK: 'togglePlayback',
  STOP_PLAYBACK: 'stopPlayback',
  START_DOWNLOAD: 'startDownload',
  OPEN_AUTH_URL: 'openAuthUrl',
  EXCHANGE_CODE: 'exchangeCode',
  GET_USER_INFO: 'getUserInfo',
  LOGOUT: 'logout',
};

// Send IPC message and wait for response
export const sendIpcMessage = (type, payload = {}) => {
  return new Promise((resolve, reject) => {
    if (!window._ipc_) {
      reject(new Error('IPC not available'));
      return;
    }

    console.log(`Sending IPC message: ${type}`, payload);

    const messageId = window._ipc_.send(type, payload);

    const handleResponse = (event) => {
      const response = event.detail;
      console.log(`Received IPC response for ${messageId}:`, response);

      if (response.id === messageId) {
        window.removeEventListener('ipc-response', handleResponse);
        clearTimeout(timeoutId);

        if (response.type === 'error') {
          reject(new Error(response.data.message));
        } else {
          resolve(response);
        }
      }
    };

    window.addEventListener('ipc-response', handleResponse);

    const timeoutId = setTimeout(() => {
      window.removeEventListener('ipc-response', handleResponse);
      reject(new Error('IPC request timed out'));
    }, 30000);

    // Cleanup on navigation
    const cleanup = () => {
      clearTimeout(timeoutId);
      window.removeEventListener('ipc-response', handleResponse);
    };

    window.addEventListener('beforeunload', cleanup);
    return () => window.removeEventListener('beforeunload', cleanup);
  });
};

// Initialize IPC
export const initIpc = () => {
  return new Promise((resolve, reject) => {
    if (window._ipc_) {
      resolve();
      return;
    }

    // Wait for IPC to be available
    let attempts = 0;
    const checkIpc = setInterval(() => {
      attempts++;
      if (window._ipc_) {
        clearInterval(checkIpc);
        resolve();
      } else if (attempts > 50) { // 5 seconds timeout
        clearInterval(checkIpc);
        reject(new Error('IPC initialization timeout'));
      }
    }, 100);
  });
};

// Export message type constants that match backend IpcResponse enum
export const RESPONSE_TYPES = {
  SUCCESS: 'success',
  ERROR: 'error',
  SEARCH_RESULTS: 'searchResults',
  ANIME_INFO: 'animeInfo'
}; 