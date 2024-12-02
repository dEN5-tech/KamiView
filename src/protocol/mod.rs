use tokio::sync::mpsc;
use std::thread;
use std::net::TcpListener;
use tiny_http::{Server, Response};

const CALLBACK_PORTS: &[u16] = &[43823, 43824, 43825, 43826, 43827];

pub struct ProtocolHandler;

impl ProtocolHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn get_callback_url(&self) -> String {
        "kamiview://oauth/callback".to_string()
    }
}

// Make ProtocolHandler Send + Sync
unsafe impl Send for ProtocolHandler {}
unsafe impl Sync for ProtocolHandler {} 