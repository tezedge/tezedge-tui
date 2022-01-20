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

use crate::model::{EndorsementState, StateRef, TuiTableData, UiState};

use super::{create_header_bar, create_help_bar, create_pages_tabs};
pub struct MempoolScreen {}

impl MempoolScreen {
    pub fn draw_mempool_screen<B: Backend>(
        data_state: &StateRef,
        ui_state: &mut UiState,
        f: &mut Frame<B>,
    ) {
        let data_state = data_state.read().unwrap();
        let size = f.size();
        let delta_toggle = ui_state.delta_toggle;

        // TODO: placeholder for mempool page
        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(3),
                Constraint::Length(4),
            ])
            .split(size);

        let (header_chunk, summary_chunk, endorsements_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(4),
                Constraint::Min(1),
            ])
            .split(page_chunks[0])
            .into_iter()
            .collect_tuple()
            .unwrap(); // safe as we specify 3 elements in constraints and collecting into tuple of size 3

        // ======================== HEADER ========================
        let header = &data_state.current_head_header;
        create_header_bar(header_chunk, header, f);

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
            "Receive hash",
            "Receive content",
            "Decode",
            "Precheck",
            "Apply",
            "Broadcast",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();

        // add ▼ to the selected sorted table
        if let Some(v) = headers.get_mut(3) {
            *v = format!("{}▼", v)
        }

        let endorsers = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let renderable_constraints = ui_state
            .endorsement_table
            .renderable_constraints(f.size().width - 2);
        let header_cells = ui_state
            .endorsement_table
            .renderable_headers(selected_style);
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = ui_state.endorsement_table.renderable_rows(
            &data_state.current_head_endorsement_statuses,
            delta_toggle,
            selected_style,
        );

        let table = Table::new(rows)
            .header(header)
            .block(endorsers)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&renderable_constraints);
        f.render_stateful_widget(
            table,
            endorsements_chunk,
            &mut ui_state.endorsement_table.table_state.clone(),
        );

        // let block = Block::default().borders(Borders::ALL).title("Endorsements");
        // f.render_widget(block, endorsements_chunk);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(ui_state);
        f.render_widget(tabs, page_chunks[1]);

        // ======================== HELP BAR ========================
        create_help_bar(page_chunks[2], f, delta_toggle);
    }
}
