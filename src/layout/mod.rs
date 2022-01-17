pub mod syncing;
use itertools::Itertools;
pub use syncing::*;

pub mod mempool;
pub use mempool::*;

pub mod statistics;
pub use statistics::*;

use strum::IntoEnumIterator;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs, Paragraph}, Frame, backend::Backend, layout::{Layout, Direction, Constraint, Rect},
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

pub fn create_help_bar<B: Backend>(help_chunk: Rect, f: &mut Frame<B>, delta_toggle: bool) {
    let help_strings = vec![
            ("F1 - F3", "PageSwitch"),
            ("q", "Quit"),
            ("TAB", "Rotate widgets"),
            ("d", if delta_toggle {
                "Concrete values"
            } else {
                "Delta values"
            }),
            ("j", "Switch sort left"),
            ("k", "Switch sort right"),
            ("←", "Table left"),
            ("→", "Table right"),
            ("↑", "Table up"),
            ("↓", "Table down"),
        ];

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Length(24),
            Constraint::Length(24),
            Constraint::Length(24),
            Constraint::Length(24),
            Constraint::Length(24),
        ])
        .split(help_chunk);

    for (index, (row_1, row_2)) in help_strings.iter().tuple_windows().step_by(2).enumerate() {
        let p = Paragraph::new(vec![
            Spans::from(vec![
                Span::styled(format!(" {} ", row_1.0), Style::default().bg(Color::White).fg(Color::Black)),
                Span::styled(format!(" {}\n", row_1.1), Style::default())
            ]),
            Spans::from(vec![
                Span::styled(format!(" {} ", row_2.0), Style::default().bg(Color::White).fg(Color::Black)),
                Span::styled(format!(" {}\n", row_2.1), Style::default())
            ]),
        ]);
        f.render_widget(p, help_chunks[index]);
    }
}