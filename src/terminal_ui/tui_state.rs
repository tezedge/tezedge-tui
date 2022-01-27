use crossterm::event::{KeyCode, KeyModifiers};
use strum_macros::{Display, EnumIter};
use tui::widgets::TableState;

#[derive(Debug, Clone, Default)]
pub struct UiState {
    // TUI states - states refering to the view part
    pub peer_table_state: TableState,
    // pub period_info_state: PeriodInfoState,
    pub active_page: ActivePage,
    pub active_widget: ActiveWidget,
    pub current_details_length: usize,
    pub screen_width: u16,
}

#[derive(Debug, Clone)]
pub enum ActiveWidget {
    PeriodInfo,
    PeerTable,
    EndorserTable,
    StatisticsMainTable,
    StatisticsDetailsTable,
}

// TODO: make enum contain the screen struct?
#[derive(Debug, Clone, EnumIter, Display)]
pub enum ActivePage {
    Synchronization,
    Mempool,
    Statistics,
}

impl ActivePage {
    pub fn to_index(&self) -> usize {
        match self {
            ActivePage::Synchronization => 0,
            ActivePage::Mempool => 1,
            ActivePage::Statistics => 2,
        }
    }
}

impl Default for ActivePage {
    fn default() -> Self {
        ActivePage::Mempool
    }
}

impl Default for ActiveWidget {
    fn default() -> Self {
        ActiveWidget::PeriodInfo
    }
}

pub enum TuiEvent {
    Input(KeyCode, KeyModifiers),
    Resize,
    Mouse,
    Tick,
}
