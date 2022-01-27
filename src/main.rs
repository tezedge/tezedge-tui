use automaton::AutomatonManager;
use clap::Parser;
use std::error::Error;

pub mod automaton;
pub mod common;
pub mod configuration;
pub mod endorsements;
pub mod extensions;
pub mod operations;
pub mod rpc;
pub mod services;
pub mod synchronization;
pub mod terminal_ui;
pub mod websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tui_args = configuration::TuiArgs::parse();

    let mut automaton_manager = AutomatonManager::new(
        tui_args.node,
        tui_args.websocket,
        create_file_logger("tui.log"),
    );
    automaton_manager.start().await;

    Ok(())
}

fn create_file_logger(path: &str) -> slog::Logger {
    use slog::Drain;

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to create log file");

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, slog::o!())
}
