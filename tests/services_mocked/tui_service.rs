use tezedge_tui::services::tui_service::TuiService;
use tui::{backend::TestBackend, Terminal};

pub struct TuiServiceMocked {
    pub terminal: Terminal<TestBackend>,
}

impl TuiServiceMocked {
    pub fn new() -> Self {
        let backend = TestBackend::new(400, 400);

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
    type Be = TestBackend;
    fn terminal(&mut self) -> &mut Terminal<Self::Be> {
        &mut self.terminal
    }

    /// Restore terminal to its state before the app has launched
    fn restore_terminal(&mut self) {}
}
