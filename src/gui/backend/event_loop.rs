use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use wry::WebView;
use tokio::sync::mpsc::Receiver;
use log::{error, info};
use std::sync::Arc;
use std::sync::Mutex as StdMutex;

pub fn run_event_loop(
    event_loop: EventLoop<()>,
    webview: WebView,
    mut rx: Receiver<String>
) {
    // Create channel for script evaluation
    let (script_tx, script_rx) = std::sync::mpsc::channel();
    
    // Wrap WebView in Arc<Mutex> for thread-safe access
    let webview = Arc::new(StdMutex::new(webview));
    let webview_handle = webview.clone();

    // Spawn task to handle script evaluation
    tokio::spawn(async move {
        while let Some(script) = rx.recv().await {
            if let Err(e) = script_tx.send(script) {
                error!("Failed to send script: {}", e);
                break;
            }
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Handle script evaluation from channel
        if let Ok(script) = script_rx.try_recv() {
            // Safely access WebView through mutex
            if let Ok(webview) = webview.lock() {
                if let Err(e) = webview.evaluate_script(&script) {
                    error!("Failed to evaluate script: {}", e);
                }
            } else {
                error!("Failed to acquire WebView lock");
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("Window close requested");
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent {
                event: WindowEvent::Destroyed,
                ..
            } => {
                info!("Window destroyed");
                *control_flow = ControlFlow::Exit
            }
            _ => (),
        }
    });
}
