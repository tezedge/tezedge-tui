use std::str::FromStr;

use tui::style::Modifier;
use tui::text::Span;
use tui::widgets::Tabs;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use itertools::Itertools;

use crate::model::{EndorsementState, StateRef, UiState};

use super::create_pages_tabs;
pub struct MempoolScreen {}

impl MempoolScreen {
    pub fn draw_mempool_screen<B: Backend>(
        data_state: &StateRef,
        ui_state: &mut UiState,
        f: &mut Frame<B>,
    ) {
        let data_state = data_state.read().unwrap();
        let size = f.size();

        // TODO: placeholder for mempool page
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(5), Constraint::Length(3)])
            .split(size);

        let (header_chunk, summary_chunk, endorsements_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(4),
                Constraint::Min(1),
            ])
            .split(chunks[0])
            .into_iter()
            .collect_tuple()
            .unwrap(); // safe as we specify 3 elements in constraints and collecting into tuple of size 3

        // ======================== HEADER ========================
        // wrap the header chunk in border
        let block = Block::default().borders(Borders::ALL).title("Current Head");
        f.render_widget(block, header_chunk);

        let header = &data_state.current_head_header;

        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
            .split(header_chunk);

        let block_hash = Paragraph::new(Spans::from(format!("Block hash: {}", header.hash)))
            .block(Block::default())
            .alignment(Alignment::Left);
        f.render_widget(block_hash, header_chunks[0]);

        let block_level = Paragraph::new(format!("Level: {}", header.level))
            .block(Block::default())
            .alignment(Alignment::Left);
        f.render_widget(block_level, header_chunks[1]);

        let block_protocol = Paragraph::new(format!("Protocol: {}", header.protocol))
            .block(Block::default())
            .alignment(Alignment::Left);
        f.render_widget(block_protocol, header_chunks[2]);

        // ======================== SUMARY ========================
        let summary_elements_constraits = std::iter::repeat(Constraint::Percentage(16))
            .take(6)
            .collect::<Vec<_>>();

        let endorsement_statuses: Vec<String> = vec![
            "Missing",
            "Broadcasted",
            "Applied",
            "Prechecked",
            "Decoded",
            "Received",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();

        let sumary_blocks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(summary_elements_constraits)
            .split(summary_chunk);

        for (i, title) in endorsement_statuses.iter().enumerate() {
            let block_text = Paragraph::new(format!(
                "{}\n{}",
                title,
                data_state
                    .endoresement_status_summary
                    .get(
                        &EndorsementState::from_str(&title.to_ascii_lowercase())
                            .unwrap_or_default()
                    )
                    .unwrap_or(&0)
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black).fg(Color::White)),
            )
            .alignment(Alignment::Center);
            f.render_widget(block_text, sumary_blocks[i])
        }

        // ======================== ENDORSERS ========================
        let headers = [
            "Slots",
            "Baker",
            "Status",
            "Delta",
            "Receive",
            "Decode",
            "Precheck",
            "Apply",
            "Broadcast",
        ];

        let titles = headers
            .iter()
            .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::White))))
            .collect();

        let (sort_area, endrosement_table_area) = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(endorsements_chunk)
            .into_iter()
            .collect_tuple()
            .unwrap();

        let sort_by_tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Sort by"))
            .highlight_style(Style::default().fg(Color::Blue))
            .select(ui_state.endorsement_sorter_state.in_focus());
        f.render_widget(sort_by_tabs, sort_area);

        let endorsers = Block::default().borders(Borders::TOP);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = headers
            .iter()
            .map(|h| Cell::from(*h).style(Style::default()));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = data_state
            .current_head_endorsement_statuses
            .iter()
            .map(|item| {
                let item = item.construct_tui_table_data();
                let height = item
                    .iter()
                    .map(|content| content.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0)
                    + 1;
                let cells = item.iter().map(|c| Cell::from(c.clone()));
                Row::new(cells).height(height as u16)
            });

        let table = Table::new(rows)
            .header(header)
            .block(endorsers)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Length(4),
                Constraint::Length(36),
                Constraint::Min(11),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(9),
            ]);
        f.render_widget(table, endrosement_table_area);

        let block = Block::default().borders(Borders::ALL).title("Endorsements");
        f.render_widget(block, endorsements_chunk);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(ui_state);
        f.render_widget(tabs, chunks[1]);
    }
}
