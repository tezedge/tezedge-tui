use std::io::Stdout;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use tui::Frame;

use crate::automaton::State;
use crate::common::{create_header_bar, create_help_bar, create_pages_tabs, create_quit};
use crate::extensions::{CustomSeparator, Renderable};

use super::{ApplicationSummary, BakingSummary, BlockApplicationSummary};

// TODO: will this be the actual homescreen?
pub struct BakingScreen {}

impl Renderable for BakingScreen {
    fn draw_screen(state: &State, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let size = f.size();
        let delta_toggle = state.delta_toggle;

        let current_block_hash = state.current_head_header.hash.clone();

        // TODO: maybe we just leave this on default
        // set the bacground color
        let background = Block::default().style(Style::default().bg(Color::Rgb(31, 30, 30)));
        f.render_widget(background, size);

        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(size);

        let (baking_table_chunk, summary_chunk) = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(page_chunks[1])
            .into_iter()
            .collect_tuple()
            .unwrap();

        // ======================== SUMMARY PANEL (right) ========================

        let application_summary = if let Some(application_statistics) =
            state.baking.application_statistics.get(&current_block_hash)
        {
            BlockApplicationSummary::from(application_statistics.clone())
        } else {
            BlockApplicationSummary::default()
        };

        let (
            summary_baking_title_chunk,
            summary_baking_level_chunk,
            summary_baking_inner_chunk,
            summary_title_chunk,
            summary_inner_chunk,
        ) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Min(15),
                Constraint::Length(4),
                Constraint::Min(17),
            ])
            .split(summary_chunk)
            .into_iter()
            .collect_tuple()
            .unwrap();

        let summary_title = Paragraph::new(Span::styled(
            " APPLICATION PROGRESS",
            Style::default().fg(Color::White),
        ))
        .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));

        f.render_widget(summary_title, summary_title_chunk);

        let selected_style = Style::default()
            .remove_modifier(Modifier::DIM)
            .bg(Color::Black);
        let normal_style = Style::default().fg(Color::White);

        let mut application_stats_table_data = application_summary.to_table_data();

        let per_peer_stats = if let Some(per_peer_stats) = state
            .baking
            .per_peer_block_statistics
            .get(&current_block_hash)
        {
            per_peer_stats.clone()
        } else {
            Vec::new()
        };

        let baking_summary = ApplicationSummary::from(per_peer_stats.clone());
        baking_summary.extend_table_data(&mut application_stats_table_data);

        let rows = application_stats_table_data
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, (tag, styled_time))| {
                let height = 1;

                let sequence_num = Cell::from(index.to_string());
                let tag = Cell::from(tag);
                let value = Cell::from(styled_time.get_string_representation())
                    .style(styled_time.get_style().remove_modifier(Modifier::DIM));

                // stripes to differentiate between lines
                if index % 2 == 0 {
                    Row::new(vec![sequence_num, tag, value])
                        .height(height)
                        .style(selected_style)
                } else {
                    Row::new(vec![sequence_num, tag, value]).height(height)
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
        f.render_widget(table, summary_inner_chunk);

        // ======================== SUMMRAY PANEL BAKING ========================
        // TODO: only update this on Baking

        let current_head_level = state.current_head_header.level;
        let current_head_timestamp = state.current_head_header.timestamp;
        let next_baking = state.baking.baking_rights.next_baking(
            current_head_level,
            &current_head_timestamp,
            state.network_constants.minimal_block_delay,
        );

        let (next_baking_time_label, next_baking_delta_label) =
            if let Some((level, time)) = next_baking {
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
            Span::styled(" BAKING PROGRESS - ", Style::default().fg(Color::White)),
            Span::styled("Next baking at level ", summary_dimmed_text_style),
            next_baking_time_label,
            Span::styled("in ", summary_dimmed_text_style),
            next_baking_delta_label,
        ]))
        .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT));

        f.render_widget(summary_title, summary_baking_title_chunk);

        let last_baked_block_level_label =
            if let Some(last_baked_block_level) = state.baking.last_baked_block_level {
                last_baked_block_level.to_string()
            } else {
                String::from(" - ")
            };

        let last_baked_block_label = Paragraph::new(Spans::from(vec![
            Span::styled(" LAST BAKED LEVEL ", Style::default().fg(Color::White)),
            Span::styled(
                last_baked_block_level_label,
                Style::default().fg(Color::White),
            ),
        ]))
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));

        f.render_widget(last_baked_block_label, summary_baking_level_chunk);

        let next_baking = state.baking.baking_rights.next_baking(
            current_head_level,
            &current_head_timestamp,
            state.network_constants.minimal_block_delay,
        );
        if let Some((level, _)) = next_baking {
            // Only update on new baking
            let baking_summary = if level == current_head_level {
                BakingSummary::new(
                    current_head_level,
                    state.previous_head_header.clone(),
                    application_summary,
                    per_peer_stats,
                )
            } else {
                state.baking.last_baking_summary.clone()
            };

            let rows = baking_summary
                .to_table_data()
                .clone()
                .into_iter()
                .enumerate()
                .map(|(index, (tag, styled_time))| {
                    let height = 1;

                    let sequence_num = Cell::from(index.to_string());
                    let tag = Cell::from(tag);
                    let value = Cell::from(styled_time.get_string_representation())
                        .style(styled_time.get_style().remove_modifier(Modifier::DIM));

                    // stripes to differentiate between lines
                    if index % 2 == 0 {
                        Row::new(vec![sequence_num, tag, value])
                            .height(height)
                            .style(selected_style)
                    } else {
                        Row::new(vec![sequence_num, tag, value]).height(height)
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
            f.render_widget(table, summary_baking_inner_chunk);
        }

        // ======================== BAKING TABLE (help) ========================

        let (help_chunk, baking_table_inner_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(5)])
            .split(baking_table_chunk)
            .into_iter()
            .collect_tuple()
            .unwrap();

        create_help_bar(help_chunk, f, delta_toggle);

        // ======================== BAKING TABLE (table) ========================

        let baking_table_block = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().remove_modifier(Modifier::DIM);
        let normal_style = Style::default().fg(Color::White);

        let renderable_constraints = state
            .baking
            .baking_table
            .renderable_constraints(calculate_percentage(f.size().width, 60));
        let header_cells = state.baking.baking_table.renderable_headers(selected_style);
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = state
            .baking
            .baking_table
            .renderable_rows(&state.baking.baking_table.content, delta_toggle);

        let highlight_symbol = "▶".to_string().to_ascii_uppercase();

        let table = Table::new(rows)
            .header(header)
            .block(baking_table_block)
            .highlight_style(selected_style)
            .highlight_symbol(&highlight_symbol)
            .widths(&renderable_constraints);
        f.render_stateful_widget(
            table,
            baking_table_inner_chunk,
            &mut state.baking.baking_table.table_state.clone(),
        );

        // overlap the block corners with special separators to make flush transition to the table block
        let vertical_left_separator = CustomSeparator::default()
            .separator("├")
            .corner(Corner::TopLeft);
        f.render_widget(vertical_left_separator, baking_table_inner_chunk);

        let vertical_right_separator = CustomSeparator::default()
            .separator("┤")
            .corner(Corner::TopRight);
        f.render_widget(vertical_right_separator, baking_table_inner_chunk);

        // let histogram_data = state.baking.per_peer_block_statistics.to_histogram_data();

        // let histogram: Vec<(&str, u64)> = histogram_data
        //     .iter()
        //     .map(|(label, value)| (&**label, *value))
        //     .collect();

        // let (chart_area, barchart_area) = Layout::default()
        //     .direction(Direction::Horizontal)
        //     .constraints([Constraint::Length(8), Constraint::Min(1)])
        //     .split(page_chunks[2])
        //     .into_iter()
        //     .collect_tuple()
        //     .unwrap();

        // let dummy_dataset = Dataset::default()
        //     .data(&[(5.0, 0.0)]);

        // let chart = Chart::new(Vec::new()).y_axis(
        //     Axis::default()
        //         .bounds([0.0, 10.0])
        //         .labels(
        //             vec![
        //                 Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
        //                 Span::styled("5", Style::default().add_modifier(Modifier::BOLD)),
        //                 Span::styled("10", Style::default().add_modifier(Modifier::BOLD))
        //             ]
        //         )
        // );
        // f.render_widget(chart, chart_area);

        // let barchart = BarChart::default()
        //     .data(histogram.as_slice())
        //     .max(10)
        //     .bar_width(8)
        //     .value_style(Style::default().fg(Color::Black).bg(Color::Cyan))
        //     .bar_style(Style::default().fg(Color::Cyan));

        // // let test = Paragraph::new(format!("{:?}", histogram));
        // f.render_widget(barchart, barchart_area);

        // ======================== HEADER ========================
        create_header_bar(page_chunks[0], state, f);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(&state.ui);
        f.render_widget(tabs, page_chunks[2]);

        // ======================== Quit ========================
        create_quit(page_chunks[2], f);
    }
}

fn calculate_percentage(val: u16, perc: u16) -> u16 {
    (val * perc) / 100
}
