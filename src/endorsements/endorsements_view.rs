use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::Corner;
use tui::style::Modifier;
use tui::text::{Span, Spans};
use tui::widgets::Cell;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::automaton::State;
use crate::common::{create_header_bar, create_help_bar, create_pages_tabs, create_quit};
use crate::extensions::{CustomSeparator, Renderable};

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

        let (header_chunk, summary_chunk, endorsements_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(page_chunks[0])
            .into_iter()
            .collect_tuple()
            .unwrap(); // safe as we specify 3 elements in constraints and collecting into tuple of size 3

        let (endorsement_table_chunk, endorsing_panel_chunk) = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(65),
                Constraint::Percentage(35),
            ])
            .split(endorsements_chunk)
            .into_iter()
            .collect_tuple()
            .unwrap();

        let (endorsement_table_help_chunk, endorsement_table_inner_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(1),
            ])
            .split(endorsement_table_chunk)
            .into_iter()
            .collect_tuple()
            .unwrap();
        
        
        // ======================== HEADER ========================
        create_header_bar(header_chunk, state, f);

        // ======================== SUMARY ========================
        let separator = Span::styled(
            " —",
            Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
        );

        let filled_style = Style::default().fg(Color::White);
        let empty_style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);

        let mut summary: Vec<Span> = EndorsementState::iter()
            .map(|endorsement_status| {
                let (styled_count, caption_style) = if let Some(count) = state
                    .endorsmenents
                    .endoresement_status_summary
                    .get(&endorsement_status)
                {
                    (
                        Span::styled(count.to_string(), endorsement_status.get_style_fg()),
                        filled_style,
                    )
                } else {
                    (
                        Span::styled(String::from("0"), endorsement_status.get_style_fg()),
                        empty_style,
                    )
                };

                vec![
                    Span::styled(
                        format!(" {}: ", endorsement_status.to_string()),
                        caption_style,
                    ),
                    styled_count,
                    separator.clone(),
                ]
            })
            .flatten()
            .collect();

        // remove the last separator
        summary.pop();

        let summary_paragraph = Paragraph::new(Spans::from(summary));

        f.render_widget(summary_paragraph, summary_chunk);

        // ======================== HELP BAR ========================
        create_help_bar(endorsement_table_help_chunk, f, delta_toggle);

        // ======================== ENDORSERS ========================

        let endorsers = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().remove_modifier(Modifier::DIM);
        let normal_style = Style::default().fg(Color::White);

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

        let rows = state
            .endorsmenents
            .endorsement_table
            .renderable_rows(&state.endorsmenents.endorsement_table.content, delta_toggle);

        let highlight_symbol = "▶".to_string().to_ascii_uppercase();

        let table = Table::new(rows)
            .header(header)
            .block(endorsers)
            .highlight_style(selected_style)
            .highlight_symbol(&highlight_symbol)
            .widths(&renderable_constraints);
        f.render_stateful_widget(
            table,
            endorsement_table_inner_chunk,
            &mut state.endorsmenents.endorsement_table.table_state.clone(),
        );

        // overlap the block corners with special separators to make flush transition to the table block
        let vertical_left_separator = CustomSeparator::default()
            .separator("├")
            .corner(Corner::TopLeft);
        f.render_widget(vertical_left_separator, endorsement_table_inner_chunk);

        let vertical_right_separator = CustomSeparator::default()
            .separator("┤")
            .corner(Corner::TopRight);
        f.render_widget(vertical_right_separator, endorsement_table_inner_chunk);

        // let block = Block::default().borders(Borders::ALL).title("Endorsements");
        // f.render_widget(block, endorsements_chunk);

        // ======================== BAKER ENDORSING PANEL ========================
        let (endorsing_panel_title_chunk, endorsing_panel_inner_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(endorsing_panel_chunk)
            .into_iter()
            .collect_tuple()
            .unwrap();

        let endorser_panel_title = Paragraph::new(Spans::from(vec![
            Span::styled(
                " ENDORSING PROGRESS ",
                Style::default().fg(Color::White),
            ),
        ]))
        .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));

        f.render_widget(endorser_panel_title, endorsing_panel_title_chunk);


        let rows = to_table_data_mocked()
            .into_iter()
            .enumerate()
            .map(|(index, (tag, value))| {
                let sequence_num_cell = Cell::from(index.to_string());
                let tag_cell = Cell::from(tag);
                let value_cell = Cell::from(value.clone());

                // TODO: need more elegant solution
                if value != *" - " {
                    Row::new(vec![sequence_num_cell, tag_cell, value_cell])
                        .height(1)
                        .style(selected_style)
                } else {
                    Row::new(vec![sequence_num_cell, tag_cell, value_cell]).height(1)
                }
            });

        let block = Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT);
        let table = Table::new(rows)
            // .header(header)
            .block(block)
            .widths(&[
                Constraint::Length(2),
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ]);
        f.render_widget(table, endorsing_panel_inner_chunk);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(&state.ui);
        f.render_widget(tabs, page_chunks[1]);

        // ======================== Quit ========================
        create_quit(page_chunks[1], f);
    }
}

// TODO -> get from the data once the rpc is completed
pub fn to_table_data_mocked() -> Vec<(Spans<'static>, String)> {
    vec![
        (Spans::from("Injected"), String::from(" - ")),
        (Spans::from("Prechecked"), String::from(" - ")),
        (Spans::from("Operation Hash Sent"), String::from(" - ")),
        (Spans::from("Operation Requested"), String::from(" - ")),
        (Spans::from("Operation Sent"), String::from(" - ")),
        (Spans::from("Operation Hash Received back"), String::from(" - ")),
    ]
}