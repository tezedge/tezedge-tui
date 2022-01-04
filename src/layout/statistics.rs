use std::str::FromStr;

use tui::style::Modifier;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use itertools::Itertools;

use crate::model::{UiState, StateRef};

use super::create_pages_tabs;
pub struct StatisticsScreen {}

impl StatisticsScreen {
    pub fn draw_statistics_screen<B: Backend>(
        data_state: &StateRef,
        ui_state: &mut UiState,
        f: &mut Frame<B>,
    ) {
        
        let size = f.size();

        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(5), Constraint::Length(3)])
            .split(size);

        // ======================== MAIN STATISTICS TABLE ========================
        
        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(ui_state);
        f.render_widget(tabs, page_chunks[1]);
    }
}