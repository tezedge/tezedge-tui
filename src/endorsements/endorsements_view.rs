use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::style::Modifier;
use tui::text::{Span, Spans};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use itertools::Itertools;

use crate::automaton::State;
use crate::common::{create_header_bar, create_help_bar, create_pages_tabs};
use crate::extensions::Renderable;

use super::EndorsementState;
pub struct EndorsementsScreen {}

impl Renderable for EndorsementsScreen {
    fn draw_screen(state: &State, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let size = f.size();
        let delta_toggle = state.delta_toggle;

        let background = Block::default().style(Style::default().bg(Color::Rgb(31, 30, 30)));
        f.render_widget(background, size);

        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(1)])
            .split(size);

        let (header_chunk, summary_chunk, help_chunk, endorsements_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Min(1),
            ])
            .split(page_chunks[0])
            .into_iter()
            .collect_tuple()
            .unwrap(); // safe as we specify 3 elements in constraints and collecting into tuple of size 3

        // ======================== HEADER ========================
        let header = &state.current_head_header;
        create_header_bar(header_chunk, header, f);

        // ======================== SUMARY ========================

        let separator = Span::styled(" — ", Style::default().fg(Color::Gray).add_modifier(Modifier::DIM));

        let filled_style = Style::default().fg(Color::White);
        let empty_style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);

        // TODO: make an iteration?
        let (missing_count, missing_style) = if let Some(count) = state
            .endorsmenents
            .endoresement_status_summary
            .get(&EndorsementState::Missing)
        {
            (count.to_string(), filled_style)
        } else {
            (String::from("0"), empty_style)
        };

        let (broadcast_count, broadcast_style) = if let Some(count) = state
            .endorsmenents
            .endoresement_status_summary
            .get(&EndorsementState::Broadcast)
        {
            (count.to_string(), filled_style)
        } else {
            (String::from("0"), empty_style)
        };

        let (applied_count, applied_style) = if let Some(count) = state
            .endorsmenents
            .endoresement_status_summary
            .get(&EndorsementState::Applied)
        {
            (count.to_string(), filled_style)
        } else {
            (String::from("0"), empty_style)
        };

        let (prechecked_count, prechecked_style) = if let Some(count) = state
            .endorsmenents
            .endoresement_status_summary
            .get(&EndorsementState::Prechecked)
        {
            (count.to_string(), filled_style)
        } else {
            (String::from("0"), empty_style)
        };

        let (decoded_count, decoded_style) = if let Some(count) = state
            .endorsmenents
            .endoresement_status_summary
            .get(&EndorsementState::Decoded)
        {
            (count.to_string(), filled_style)
        } else {
            (String::from("0"), empty_style)
        };

        let (received_count, received_style) = if let Some(count) = state
            .endorsmenents
            .endoresement_status_summary
            .get(&EndorsementState::Received)
        {
            (count.to_string(), filled_style)
        } else {
            (String::from("0"), empty_style)
        };

        let summary = Paragraph::new(Spans::from(vec![
            Span::styled("Missing: ", missing_style),
            Span::styled(missing_count, EndorsementState::Missing.get_style_fg()),
            separator.clone(),
            Span::styled("Broadcast: ", broadcast_style),
            Span::styled(broadcast_count, EndorsementState::Broadcast.get_style_fg()),
            separator.clone(),
            Span::styled("Applied: ", applied_style),
            Span::styled(applied_count, EndorsementState::Applied.get_style_fg()),
            separator.clone(),
            Span::styled("Prechecked: ", prechecked_style),
            Span::styled(
                prechecked_count,
                EndorsementState::Prechecked.get_style_fg(),
            ),
            separator.clone(),
            Span::styled("Decoded: ", decoded_style),
            Span::styled(decoded_count, EndorsementState::Decoded.get_style_fg()),
            separator,
            Span::styled("Received: ", received_style),
            Span::styled(received_count, EndorsementState::Received.get_style_fg()),
        ]));

        f.render_widget(summary, summary_chunk);

        // ======================== HELP BAR ========================
        create_help_bar(help_chunk, f, delta_toggle);

        // ======================== ENDORSERS ========================

        let endorsers = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().remove_modifier(Modifier::DIM);
        let normal_style = Style::default().fg(Color::Gray);

        let renderable_constraints = state
            .endorsmenents
            .endorsement_table
            .renderable_constraints(f.size().width - 2);
        let header_cells = state
            .endorsmenents
            .endorsement_table
            .renderable_headers(selected_style);
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = state.endorsmenents.endorsement_table.renderable_rows(
            &state.endorsmenents.current_head_endorsement_statuses,
            delta_toggle,
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
            &mut state.endorsmenents.endorsement_table.table_state.clone(),
        );

        // let block = Block::default().borders(Borders::ALL).title("Endorsements");
        // f.render_widget(block, endorsements_chunk);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(&state.ui);
        f.render_widget(tabs, page_chunks[1]);
    }
}
