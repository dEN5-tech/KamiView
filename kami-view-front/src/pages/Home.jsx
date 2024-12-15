import { useEffect } from 'preact/hooks';
import { motion } from 'framer-motion';
import { Card } from '../components/ui/Card';
import { sendIpcMessage, IPC_TYPES } from '../utils/ipc';

export function Home() {
  useEffect(() => {
    // Test IPC connection on component mount
    const testConnection = async () => {
      try {
        await sendIpcMessage(IPC_TYPES.SYSTEM, { action: 'ready_check' });
        console.log('IPC connection established');
      } catch (err) {
        console.error('IPC connection failed:', err);
      }
    };
    testConnection();
  }, []);

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="container mx-auto px-6 py-8"
    >
      <h1 className="text-2xl font-bold mb-6">Welcome to KamiView</h1>
      <Card className="p-6">
        <p>Start by searching for your favorite anime!</p>
      </Card>
    </motion.div>
  );
} 