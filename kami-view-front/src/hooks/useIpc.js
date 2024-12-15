export function useIpc() {
  const sendMessage = async (type, data = {}) => {
    if (!window.ipc) {
      throw new Error('IPC not available');
    }

    return new Promise((resolve, reject) => {
      const messageId = Date.now().toString();
      const message = {
        id: messageId,
        type,
        data,
      };

      const handleResponse = (event) => {
        const response = event.detail;
        if (response.id === messageId) {
          if (response.error) {
            reject(new Error(response.error));
          } else {
            resolve(response.data);
          }
          cleanup();
        }
      };

      const cleanup = () => {
        window.removeEventListener('ipc-response', handleResponse);
      };

      window.addEventListener('ipc-response', handleResponse);

      window.ipc.postMessage(JSON.stringify(message));

      setTimeout(() => {
        cleanup();
        reject(new Error('IPC request timed out'));
      }, 10000);
    });
  };

  return { sendMessage };
} 