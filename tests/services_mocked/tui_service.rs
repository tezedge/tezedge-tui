use std::io::{Stdout, self};

use tezedge_tui::services::tui_service::TuiService;
use tui::{Terminal, backend::CrosstermBackend};



pub struct TuiServiceMocked {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TuiServiceMocked {
    pub fn new() -> Self {
        let stdout = io::stdout();

        let backend = CrosstermBackend::new(stdout);

        let terminal = Terminal::new(backend).expect("Error initializing terminal");

        TuiServiceMocked { terminal }
    }
}

impl Default for TuiServiceMocked {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiService for TuiServiceMocked {
    fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }

    /// Restore terminal to its state before the app has launched
    fn restore_terminal(&mut self) {}
}