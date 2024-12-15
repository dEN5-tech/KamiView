use std::sync::Arc;
use std::thread;
use tiny_http::{Server, Response, Header};
use port_scanner::local_port_available;
use log::{info, error};

pub struct LocalServer {
    port: u16,
    html: Arc<String>,
}

impl LocalServer {
    pub fn new(html: String) -> Self {
        Self {
            port: 0,
            html: Arc::new(html),
        }
    }

    pub fn start(&mut self) -> String {
        // Find available port
        let port = (3000..4000)
            .find(|&port| local_port_available(port))
            .expect("No available ports found");
        
        self.port = port;
        let html = self.html.clone();

        info!("Starting local server on port {}", port);

        thread::spawn(move || {
            let server = match Server::http(format!("127.0.0.1:{}", port)) {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to start server: {}", e);
                    return;
                }
            };

            info!("Local server started successfully");

            for request in server.incoming_requests() {
                let content_type = "text/html; charset=utf-8";
                let header = Header::from_bytes("Content-Type", content_type)
                    .expect("Failed to create content-type header");

                let response = Response::from_string(html.to_string())
                    .with_header(header);

                if let Err(e) = request.respond(response) {
                    error!("Failed to send response: {}", e);
                }
            }
        });

        format!("http://127.0.0.1:{}", port)
    }

    #[allow(dead_code)]
    pub fn get_port(&self) -> u16 {
        self.port
    }
} 