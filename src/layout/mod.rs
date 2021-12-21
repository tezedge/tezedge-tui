pub mod syncing;
pub use syncing::*;

pub mod mempool;
pub use mempool::*;

use strum::IntoEnumIterator;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
};

use crate::model::{ActivePage, UiState};

pub fn create_pages_tabs(ui_state: &UiState) -> Tabs {
    let titles = ActivePage::iter()
        .map(|t| {
            Spans::from(vec![
                // Span::styled(
                //     t.shortcut.clone(),
                //     Style::default().fg(Color::Yellow).bg(Color::Black),
                // ),
                Span::styled(
                    t.to_string(),
                    Style::default().fg(Color::White).bg(Color::Black),
                ),
            ])
        })
        .collect();
    let page_in_focus = ui_state.active_page.to_index();
    Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Gray))
        .select(page_in_focus)
}
