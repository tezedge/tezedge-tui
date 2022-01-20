pub mod syncing;
use itertools::Itertools;
pub use syncing::*;

pub mod mempool;
pub use mempool::*;

pub mod statistics;
pub use statistics::*;

use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::model::{ActivePage, CurrentHeadHeader, UiState};

pub fn create_pages_tabs(ui_state: &UiState) -> Tabs {
    let titles = ActivePage::iter()
        .map(|t| {
            Spans::from(vec![Span::styled(
                t.to_string(),
                Style::default().fg(Color::White).bg(Color::Black),
            )])
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
        (
            "d",
            if delta_toggle {
                "Concrete values"
            } else {
                "Delta values"
            },
        ),
        ("s", "Sort ascending"),
        ("^s", "Sort descending"),
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
                Span::styled(
                    format!(" {} ", row_1.0),
                    Style::default().bg(Color::White).fg(Color::Black),
                ),
                Span::styled(format!(" {}\n", row_1.1), Style::default()),
            ]),
            Spans::from(vec![
                Span::styled(
                    format!(" {} ", row_2.0),
                    Style::default().bg(Color::White).fg(Color::Black),
                ),
                Span::styled(format!(" {}\n", row_2.1), Style::default()),
            ]),
        ]);
        f.render_widget(p, help_chunks[index]);
    }
}

pub fn create_header_bar<B: Backend>(
    header_chunk: Rect,
    header: &CurrentHeadHeader,
    f: &mut Frame<B>,
) {
    // wrap the header info in borders
    let block = Block::default().borders(Borders::ALL).title("Current Head");
    f.render_widget(block, header_chunk);

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
        .split(header_chunk);

    let block_hash = Paragraph::new(Spans::from(vec![
        Span::styled("Block hash: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{} ", header.hash),
            Style::default().fg(Color::Reset),
        ),
    ]));

    f.render_widget(block_hash, header_chunks[0]);

    let block_level = Paragraph::new(Spans::from(vec![
        Span::styled("Level: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{} ", header.level),
            Style::default().fg(Color::Reset),
        ),
    ]));

    f.render_widget(block_level, header_chunks[1]);

    let block_protocol = Paragraph::new(Spans::from(vec![
        Span::styled("Protocol: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{} ", header.protocol),
            Style::default().fg(Color::Reset),
        ),
    ]));

    f.render_widget(block_protocol, header_chunks[2]);
}
