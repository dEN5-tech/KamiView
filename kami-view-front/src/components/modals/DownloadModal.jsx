import { useEffect, useState } from 'preact/hooks';
import { useSelector, useDispatch } from 'react-redux';
import { setError, setDownloadModal, setDownloadInfo } from '../../store/slices/animeDetailsSlice';

export function DownloadModal({ isOpen, onClose, downloadInfo }) {
  const dispatch = useDispatch();
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [downloadStatus, setDownloadStatus] = useState('preparing'); // preparing, downloading, completed, error
  const [downloadError, setDownloadError] = useState(null);

  useEffect(() => {
    if (isOpen && downloadInfo) {
      setDownloadStatus('preparing');
      setDownloadProgress(0);
      setDownloadError(null);
      startDownload();
    }
  }, [isOpen, downloadInfo]);

  // Listen for download messages from backend
  useEffect(() => {
    if (!window.ipc) return;

    const handleDownloadMessage = (msg) => {
      try {
        const data = JSON.parse(msg);
        switch (data.type) {
          case 'DownloadProgress':
            setDownloadProgress(data.data.percent);
            setDownloadStatus('downloading');
            break;
          case 'DownloadComplete':
            setDownloadProgress(100);
            setDownloadStatus('completed');
            break;
          case 'DownloadError':
            setDownloadError(data.data.message);
            setDownloadStatus('error');
            dispatch(setError(data.data.message));
            break;
        }
      } catch (e) {
        console.error('Failed to parse download message:', e);
      }
    };

    const originalCallback = window.__IPC_CALLBACK__;
    window.__IPC_CALLBACK__ = (msg) => {
      handleDownloadMessage(msg);
      if (originalCallback) originalCallback(msg);
    };

    return () => {
      window.__IPC_CALLBACK__ = originalCallback;
    };
  }, []);

  const handleClose = () => {
    if (downloadStatus !== 'downloading') {
      dispatch(setDownloadModal(false));
      dispatch(setDownloadInfo(null));
      onClose?.();
    }
  };

  const startDownload = () => {
    if (!downloadInfo) return;
    
    setDownloadStatus('downloading');
    window.ipc.postMessage(JSON.stringify({
      type: 'startDownload',
      data: {
        content: downloadInfo.content,
        filename: downloadInfo.filename,
        contentType: downloadInfo.contentType || 'application/x-mpegURL'
      }
    }));
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-card rounded-lg p-6 max-w-md w-full mx-4 shadow-xl">
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-lg font-medium">Downloading File</h3>
          <button 
            onClick={handleClose}
            disabled={downloadStatus === 'downloading'}
            className={`text-gray-400 hover:text-white transition-colors
              ${downloadStatus === 'downloading' ? 'opacity-50 cursor-not-allowed' : ''}
            `}
          >
            <i className="fas fa-times" />
          </button>
        </div>

        <div className="mb-4">
          <p className="text-sm text-gray-400 mb-2">
            {downloadInfo?.filename}
          </p>
          <div className="h-2 bg-black/20 rounded-full overflow-hidden">
            <div 
              className="h-full bg-primary transition-all duration-300"
              style={{ width: `${downloadProgress}%` }}
            />
          </div>
        </div>

        <div className="flex items-center justify-between">
          <span className="text-sm text-gray-400">
            {downloadStatus === 'preparing' && 'Preparing download...'}
            {downloadStatus === 'downloading' && `${downloadProgress}% completed`}
            {downloadStatus === 'completed' && 'Download completed'}
            {downloadStatus === 'error' && (downloadError || 'Download failed')}
          </span>
          <div className="flex gap-2">
            {downloadStatus === 'error' && (
              <button
                onClick={startDownload}
                className="px-4 py-2 bg-primary rounded-lg hover:bg-primary/90 transition-colors"
              >
                Retry
              </button>
            )}
            <button
              onClick={handleClose}
              disabled={downloadStatus === 'downloading'}
              className={`px-4 py-2 bg-white/5 rounded-lg hover:bg-white/10 transition-colors
                ${downloadStatus === 'downloading' ? 'opacity-50 cursor-not-allowed' : ''}
              `}
            >
              {downloadStatus === 'completed' ? 'Close' : 'Cancel'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
} 