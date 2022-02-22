use strum::IntoEnumIterator;
use time::Duration;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::{
    automaton::State,
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

pub fn create_header_bar<B: Backend>(header_chunk: Rect, state: &State, f: &mut Frame<B>) {
    let header = &state.current_head_header;
    // wrap the header info in borders
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().add_modifier(Modifier::DIM));
    f.render_widget(block, header_chunk);

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(23),
            Constraint::Length(22),
            Constraint::Length(23),
            Constraint::Length(18),
            Constraint::Min(50),
        ])
        .split(header_chunk);

    let block_hash_short = if !header.hash.is_empty() {
        let start = header.hash.chars().take(6).collect::<String>();
        let end = header.hash.chars().rev().take(6).collect::<String>();
        format!("{}..{}", start, end)
    } else {
        String::from("")
    };
    let block_hash = Paragraph::new(Spans::from(vec![
        Span::styled(
            " Block: ",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        ),
        Span::styled(
            format!("{} ", block_hash_short),
            Style::default().fg(Color::White),
        ),
    ]));

    f.render_widget(block_hash, header_chunks[0]);

    let block_level = Paragraph::new(Spans::from(vec![
        Span::styled(
            "Local Level: ",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        ),
        Span::styled(
            format!("{} ", header.level),
            Style::default().fg(Color::White),
        ),
    ]));

    f.render_widget(block_level, header_chunks[1]);

    let remote_level = Paragraph::new(Spans::from(vec![
        Span::styled(
            "Remote Level: ",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        ),
        Span::styled(
            format!("{} ", state.best_remote_level.unwrap_or_default()),
            Style::default().fg(Color::White),
        ),
    ]));

    f.render_widget(remote_level, header_chunks[2]);

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

    f.render_widget(block_protocol, header_chunks[3]);

    let baker_info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(21), Constraint::Length(26)])
        .split(header_chunks[4]);

    // render next baking endorsements if we have baker address specified
    if state.baker_address.is_some() {
        // Baking in 59 minutes
        // Endorsement in 59 minutes

        let baking_in = if let Some((_, time)) = state.baking.baking_rights.next_baking(
            header.level,
            &header.timestamp,
            state.network_constants.minimal_block_delay,
        ) {
            time
        } else {
            String::from("Never")
        };

        let baking = Paragraph::new(Spans::from(vec![
            Span::styled(
                "Baking in ",
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::styled(format!("{} ", baking_in), Style::default().fg(Color::White)),
        ]))
        .alignment(Alignment::Right);

        f.render_widget(baking, baker_info_chunks[0]);

        let endorsing_in = if let Some((_, time)) = state
            .endorsmenents
            .endorsement_rights_with_time
            .next_endorsing(
                header.level + 1,
                header.timestamp.saturating_add(Duration::seconds(
                    state.network_constants.minimal_block_delay.into(),
                )),
                state.network_constants.minimal_block_delay,
            ) {
            time
        } else {
            String::from("Never")
        };

        // TODO
        let endorsing = Paragraph::new(Spans::from(vec![
            Span::styled(
                "Endorsement in ",
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ),
            Span::styled(
                format!("{} ", endorsing_in),
                Style::default().fg(Color::White),
            ),
        ]))
        .alignment(Alignment::Right);

        f.render_widget(endorsing, baker_info_chunks[1]);
    }
}

pub fn create_quit<B: Backend>(last_chunk: Rect, f: &mut Frame<B>) {
    let quit = Paragraph::new(Spans::from(vec![
        Span::styled(
            "F10",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::DIM),
        ),
        Span::styled("QUIT", Style::default().fg(Color::White)),
    ]))
    .alignment(Alignment::Right);
    f.render_widget(quit, last_chunk);
}
