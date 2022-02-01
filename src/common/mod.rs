use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::{
    services::rpc_service::CurrentHeadHeader,
    terminal_ui::{ActivePage, UiState},
};

pub fn create_pages_tabs(ui_state: &UiState) -> Tabs {
    let titles = ActivePage::iter()
        .map(|t| {
            Spans::from(vec![
                Span::styled(
                    t.hotkey(),
                    Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
                ),
                Span::styled(
                    t.to_string().to_ascii_uppercase(),
                    Style::default().fg(Color::White),
                ),
            ])
        })
        .collect();
    let page_in_focus = ui_state.active_page.to_index();
    Tabs::new(titles)
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .remove_modifier(Modifier::DIM),
        )
        .divider(" ")
        .select(page_in_focus)
}

pub fn create_help_bar<B: Backend>(help_chunk: Rect, f: &mut Frame<B>, delta_toggle: bool) {
    let help_strings = vec![
        ("←→↑↓", "Navigate Table"),
        ("s", "Sort"),
        (
            "d",
            if delta_toggle {
                "Concrete values"
            } else {
                "Delta values"
            },
        ),
        ("TAB", "Switch Focus"),
    ];

    let help_spans: Vec<Span> = help_strings
        .iter()
        .map(|(key, help)| {
            vec![
                Span::styled(*key, Style::default().fg(Color::White)),
                Span::from(" "),
                Span::styled(
                    *help,
                    Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
                ),
                Span::from(" "),
            ]
        })
        .flatten()
        .collect();

    let help_paragraph = Paragraph::new(Spans::from(help_spans))
        .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));
    f.render_widget(help_paragraph, help_chunk);
}

pub fn create_header_bar<B: Backend>(
    header_chunk: Rect,
    header: &CurrentHeadHeader,
    f: &mut Frame<B>,
) {
    // wrap the header info in borders
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().add_modifier(Modifier::DIM));
    f.render_widget(block, header_chunk);

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(62),
            Constraint::Length(16),
            Constraint::Length(18),
        ])
        .split(header_chunk);

    let block_hash = Paragraph::new(Spans::from(vec![
        Span::styled(
            " Block: ",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        ),
        Span::styled(
            format!("{} ", header.hash),
            Style::default().fg(Color::White),
        ),
    ]));

    f.render_widget(block_hash, header_chunks[0]);

    let block_level = Paragraph::new(Spans::from(vec![
        Span::styled(
            "Level: ",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        ),
        Span::styled(
            format!("{} ", header.level),
            Style::default().fg(Color::White),
        ),
    ]));

    f.render_widget(block_level, header_chunks[1]);

    // show only the shorter version of the protocol
    let protocol_short = if !header.protocol.is_empty() {
        header.protocol.split_at(8).0.to_string()
    } else {
        header.protocol.to_owned()
    };

    let block_protocol = Paragraph::new(Spans::from(vec![
        Span::styled(
            "Protocol: ",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        ),
        Span::styled(
            format!("{} ", protocol_short),
            Style::default().fg(Color::White),
        ),
    ]));

    f.render_widget(block_protocol, header_chunks[2]);
}
