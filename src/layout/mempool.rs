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
            "Broadcast",
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
        let mut headers: Vec<String> = [
            "Slots",
            "Baker",
            "Status",
            "Delta",
            "Receive",
            "Decode",
            "Precheck",
            "Apply",
            "Broadcast",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();

        // add ▼ to the selected sorted table
        headers
            .get_mut(ui_state.endorsement_sorter_state.in_focus())
            .map(|v| *v = format!("{}▼", v));

        let endorsers = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = headers
            .iter()
            .map(|h| Cell::from(h.as_str()).style(Style::default()));
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
                Constraint::Length(6),
                Constraint::Length(36),
                Constraint::Min(11),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(9),
                Constraint::Min(8),
                Constraint::Min(10),
            ]);
        f.render_widget(table, endorsements_chunk);

        // let block = Block::default().borders(Borders::ALL).title("Endorsements");
        // f.render_widget(block, endorsements_chunk);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(ui_state);
        f.render_widget(tabs, chunks[1]);
    }
}
