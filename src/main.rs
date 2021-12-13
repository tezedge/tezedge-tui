use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tokio::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::ui::Ui;

pub mod model;
pub mod ui;
pub mod websocket;
pub mod layout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut ui = Ui::default();
    let ws_handle = websocket::spawn_ws_reader(ui.state.clone()).await;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create the tui app
    let res = ui.run_tui(&mut terminal, Duration::from_secs(1)).await;

    drop(ws_handle);
    // restore the terminal after exit
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
