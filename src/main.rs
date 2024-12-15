use std::sync::Arc;
use tao::{
    event_loop::EventLoop,
    window::WindowBuilder,
};
use tokio::runtime::Runtime;
use crate::di::Container;
use crate::gui::backend::{create_webview, run_event_loop};
use log::{info, LevelFilter};
use env_logger::Builder;
use std::io::Write;
use std::fs;
use crate::storage::initialize_storage_path;

mod di;
mod gui;
mod mpv;
mod utils;
mod storage;
mod client;
mod kodik;
mod shikimori;

// In debug mode, use Vite's dev server
#[cfg(debug_assertions)]
const INDEX_HTML: &str = "http://localhost:5173";

// In release mode, use the bundled HTML
#[cfg(not(debug_assertions))]
const INDEX_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/index.html"));

fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = Builder::new();
    
    // Create logs directory
    fs::create_dir_all("logs")?;
    
    // Setup log file
    let log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/kamiview.log")?;

    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    Ok(())
}


fn main() -> wry::Result<()> {
    // Initialize logger first, before any other initialization
    if let Err(e) = setup_logger() {
        eprintln!("Failed to setup logger: {}", e);
    } else {
        info!("Starting KamiView...");
    }

    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize storage paths
    storage::initialize_storage_path();
    
    // Initialize tokio runtime
    let runtime = Runtime::new()
        .expect("Failed to create Tokio runtime");
    let _guard = runtime.enter();
    
    // Initialize container and channels
    let container = Arc::new(Container::new());
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(32);
    
    // Create window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("KamiView")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 720.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .expect("Failed to build window");

    // Create webview with container and tx
    let webview = create_webview(&window, INDEX_HTML, container.clone(), tx)?;
    
    // Run event loop
    run_event_loop(event_loop, webview, rx);
    Ok(())
}
