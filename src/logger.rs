use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

pub fn init() {
    Builder::new()
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
        .target(Target::Stdout)
        .init();
} 