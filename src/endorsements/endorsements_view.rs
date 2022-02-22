use std::io::Stdout;

use time::Duration;
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

use super::{EndorsementOperationSummary, EndorsementState};
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
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(endorsements_chunk)
            .into_iter()
            .collect_tuple()
            .unwrap();

        let (endorsement_table_help_chunk, endorsement_table_inner_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(1)])
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
        let (endorsing_panel_title_chunk, endorsing_panel_level_chunk, endorsing_panel_inner_chunk) =
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(2),
                    Constraint::Min(1),
                ])
                .split(endorsing_panel_chunk)
                .into_iter()
                .collect_tuple()
                .unwrap();

        // let endorser_panel_title = Paragraph::new(Spans::from(vec![Span::styled(
        //     " ENDORSING PROGRESS ",
        //     Style::default().fg(Color::White),
        // )]))
        // .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));

        // f.render_widget(endorser_panel_title, endorsing_panel_title_chunk);
        let current_head_level = state.current_head_header.level;
        let current_head_timestamp = &state.current_head_header.timestamp;
        // TODO: constant
        // We need to get the next endorsement even when have endorsed in the current head
        let next_endorsing = state
            .endorsmenents
            .endorsement_rights_with_time
            .next_endorsing(
                current_head_level + 1,
                current_head_timestamp.saturating_add(Duration::seconds(
                    state.network_constants.minimal_block_delay.into(),
                )),
                state.network_constants.minimal_block_delay,
            );

        let (next_endorsing_time_label, next_endorsing_delta_label) =
            if let Some((level, time)) = next_endorsing {
                let blocks_delta = level - current_head_level;
                (
                    Span::styled(format!("{} ", level), Style::default().fg(Color::White)),
                    Span::styled(
                        format!("{} ({} blocks)", time, blocks_delta),
                        Style::default().fg(Color::White),
                    ),
                )
            } else {
                (
                    Span::styled(
                        "No rights found",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::DIM),
                    ),
                    Span::from(""),
                )
            };

        let summary_dimmed_text_style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::DIM);

        let summary_title = Paragraph::new(Spans::from(vec![
            Span::styled(" ENDORSING PROGRESS - ", Style::default().fg(Color::White)),
            Span::styled("Next endorsing at level ", summary_dimmed_text_style),
            next_endorsing_time_label,
            Span::styled("in ", summary_dimmed_text_style),
            next_endorsing_delta_label,
        ]))
        .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));

        f.render_widget(summary_title, endorsing_panel_title_chunk);

        // let last_endorsed_block_level_label =
        //     if let Some(last_endorsed_block_level) = state.endorsmenents.last_endrosement_operation_level {
        //         last_baked_block_level.to_string()
        //     } else {
        //         String::from(" - ")
        //     };

        // check whether the baker has rights for the current head
        let next_endorsing = state
            .endorsmenents
            .endorsement_rights_with_time
            .next_endorsing(
                current_head_level,
                *current_head_timestamp,
                state.network_constants.minimal_block_delay,
            );

        let last_endorsement_level_string = if let Some((level, _)) = next_endorsing {
            if level == current_head_level {
                current_head_level.to_string()
            } else {
                state
                    .endorsmenents
                    .last_endrosement_operation_level
                    .to_string()
            }
        } else {
            String::from("-")
        };

        let last_baked_block_label = Paragraph::new(Spans::from(vec![
            Span::styled(
                " LAST ENDORSEMENT OPERTAION IN LEVEL ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                last_endorsement_level_string,
                Style::default().fg(Color::White),
            ),
        ]))
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

        f.render_widget(last_baked_block_label, endorsing_panel_level_chunk);

        if let Some((level, _)) = next_endorsing {
            let endorsement_summary = if level == current_head_level {
                // we receive the operation stats from node earlier than the block statistics
                // so only display the full stats when everything is ready. This would confuse the user
                if let Some(block_stats) = state
                    .baking
                    .application_statistics
                    .get(&state.current_head_header.hash)
                    .cloned()
                {
                    let op_stats = state
                        .endorsmenents
                        .injected_endorsement_stats
                        .get(&current_head_level)
                        .cloned()
                        .unwrap_or_default();
                    EndorsementOperationSummary::new(
                        *current_head_timestamp,
                        op_stats,
                        Some(block_stats),
                    )
                } else {
                    EndorsementOperationSummary::default()
                }
            } else {
                state
                    .endorsmenents
                    .last_injected_endorsement_summary
                    .clone()
            };

            let selected_style = Style::default()
                .remove_modifier(Modifier::DIM)
                .bg(Color::Black);

            let rows = endorsement_summary
                .to_table_data()
                .into_iter()
                .enumerate()
                .map(|(index, (tag, styled_time))| {
                    let sequence_num_cell = Cell::from(index.to_string());
                    let tag_cell = Cell::from(tag);
                    let value_cell = Cell::from(styled_time.get_string_representation())
                        .style(styled_time.get_style().remove_modifier(Modifier::DIM));

                    // stripes to differentiate between lines
                    if index % 2 == 0 {
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
        }

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(&state.ui);
        f.render_widget(tabs, page_chunks[1]);

        // ======================== Quit ========================
        create_quit(page_chunks[1], f);
    }
}
