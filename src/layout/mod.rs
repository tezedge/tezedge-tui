pub mod syncing;
pub use syncing::*;

pub mod mempool;
pub use mempool::*;
use tui::{widgets::{Tabs, Block, Borders}, text::{Span, Spans}, style::{Style, Color}};

use crate::model::UiState;

pub fn create_pages_tabs(ui_state: &UiState) -> Tabs {
    let titles = ui_state
        .page_state
        .pages
        .iter()
        .map(|t| {
            Spans::from(Span::styled(
                t.title.clone(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();
    let page_in_focus = ui_state.page_state.in_focus();
    Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Blue))
        .select(page_in_focus)
}
