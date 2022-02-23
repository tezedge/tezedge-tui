use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Serialize, Deserialize};
use strum_macros::{Display, EnumIter};
use tui::widgets::TableState;

use crate::extensions::TableStateDef;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiState {
    // TUI states - states refering to the view part
    #[serde(with = "TableStateDef")]
    pub peer_table_state: TableState,
    // pub period_info_state: PeriodInfoState,
    pub active_page: ActivePage,
    pub active_widget: ActiveWidget,
    pub current_details_length: usize,
    pub screen_width: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActiveWidget {
    PeriodInfo,
    PeerTable,
    EndorserTable,
    StatisticsMainTable,
    StatisticsDetailsTable,
    BakingTable,
}

// TODO: make enum contain the screen struct?
#[derive(Debug, Clone, EnumIter, Display, Deserialize, Serialize)]
pub enum ActivePage {
    Synchronization,
    Endorsements,
    Statistics,
    Baking,
}

impl ActivePage {
    pub fn to_index(&self) -> usize {
        match self {
            ActivePage::Synchronization => 0,
            ActivePage::Endorsements => 1,
            ActivePage::Statistics => 2,
            ActivePage::Baking => 3,
        }
    }
    pub fn hotkey(&self) -> String {
        match self {
            ActivePage::Synchronization => String::from("F1"),
            ActivePage::Endorsements => String::from("F2"),
            ActivePage::Statistics => String::from("F3"),
            ActivePage::Baking => String::from("F4"),
        }
    }
}

impl Default for ActivePage {
    fn default() -> Self {
        ActivePage::Endorsements
    }
}

impl Default for ActiveWidget {
    fn default() -> Self {
        ActiveWidget::EndorserTable
    }
}

pub enum TuiEvent {
    Input(KeyCode, KeyModifiers),
    Resize,
    Mouse,
    Tick,
}
