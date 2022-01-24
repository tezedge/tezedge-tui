use automaton::AutomatonManager;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tokio::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

// use crate::ui::Ui;

pub mod configuration;
// pub mod layout;
// pub mod model;
// pub mod node_rpc;
// pub mod ui;
// pub mod websocket;

pub mod automaton;
pub mod endorsements;
pub mod extensions;
pub mod rpc;
pub mod services;
// pub mod ui_deprecated;
pub mod terminal_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tui_args = configuration::TuiArgs::parse();

    let mut automaton_manager = AutomatonManager::new(tui_args.node, create_file_logger("tui.log"));
    automaton_manager.start();

    std::thread::sleep(Duration::from_secs(10));

    // let mut ui = Ui::new(&tui_args);
    // let ws_handle = websocket::spawn_ws_reader(ui.state.clone(), tui_args.websocket)
    //     .await
    //     .expect("Failed to connect to websocket.");

    // Setup terminal
    // enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;

    // create the tui app
    // let res = ui.run_tui(&mut terminal, Duration::from_secs(1)).await;

    // drop(ws_handle);
    // restore the terminal after exit
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    // if let Err(err) = res {
    //     println!("{:?}", err)
    // }

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
