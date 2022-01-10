use std::time::Instant;

use slog::{info, Logger};
use tui::style::Modifier;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use itertools::Itertools;

use crate::model::{StateRef, UiState};

use super::create_pages_tabs;
pub struct StatisticsScreen {}

impl StatisticsScreen {
    pub fn draw_statistics_screen<B: Backend>(
        data_state: &StateRef,
        ui_state: &mut UiState,
        log: &Logger,
        f: &mut Frame<B>,
    ) {
        let now = Instant::now();
        let size = f.size();

        let data_state = data_state.read().unwrap();

        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(5), Constraint::Length(3)])
            .split(size);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(ui_state);
        f.render_widget(tabs, page_chunks[1]);

        let (operations_statistics, operations_statistics_sortable) =
            data_state.operations_statistics.clone();

        // Display a loading data screen until the data is loaded
        if operations_statistics.is_empty() {
            let loading = Paragraph::new("Loading data...").alignment(Alignment::Center);
            f.render_widget(loading, size);
            return;
        }

        let (main_table_chunk, details_table_chunk) = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(120), Constraint::Length(84)])
            .split(page_chunks[0])
            .into_iter()
            .collect_tuple()
            .unwrap();

        // DEBUG
        // info!(
        //     log,
        //     "Main table width: {} - Details table width: {}",
        //     main_table_chunk.width,
        //     details_table_chunk.width
        // );

        // ======================== MAIN STATISTICS TABLE ========================
        let mut main_table_headers: Vec<String> = [
            "Datetime", "Hash", "Nodes", "Delta", "Received", "Con.Rec.", "Valid.S.", "Preap.S.",
            "Preap.F.", "Valid.F.", "Val.Len.", "Sent", "Kind",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();

        // add ▼ to the selected sorted table
        if let Some(v) =
            main_table_headers.get_mut(ui_state.main_operation_statistics_sorter_state.in_focus())
        {
            *v = format!("{}▼", v)
        }

        let main_table_block = Block::default().borders(Borders::ALL).title("Operations");

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = main_table_headers
            .iter()
            .map(|h| Cell::from(h.as_str()).style(Style::default()));
        let main_table_header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = operations_statistics_sortable.iter().map(|item| {
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
            .header(main_table_header)
            .block(main_table_block)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Min(22),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(9),
                Constraint::Min(19),
            ]);

        f.render_stateful_widget(
            table,
            main_table_chunk,
            &mut ui_state.main_operation_statistics_table_state,
        );

        // ======================== DETAILS TABLE ========================

        let details_table_headers: Vec<String> = [
            "Node Id", "1.Rec.", "1.Rec.C.", "1.Sent", "Received", "Con.Rec.", "Sent",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();

        let details_table_block = Block::default().borders(Borders::ALL).title("Details");

        let header_cells = details_table_headers
            .iter()
            .map(|h| Cell::from(h.as_str()).style(Style::default()));
        let details_table_header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = if let Some(index) = ui_state.main_operation_statistics_table_state.selected() {
            let hash = operations_statistics_sortable[index].hash.clone();

            if let Some(stats) = operations_statistics.get(&hash) {
                ui_state.current_details_length = stats.node_count();
                stats.to_operations_details().into_iter().map(|v| {
                    let item = v.construct_tui_table_data();

                    let height = item
                        .iter()
                        .map(|content| content.chars().filter(|c| *c == '\n').count())
                        .max()
                        .unwrap_or(0)
                        + 1;
                    let cells = item.iter().map(|c| Cell::from(c.clone()));
                    Row::new(cells).height(height as u16)
                })
            } else {
                let details = Paragraph::new("Select an operation for details...")
                    .alignment(Alignment::Center);
                f.render_widget(details, details_table_chunk);
                return;
            }
        } else {
            // TODO: duplicate... put inside fn/clousure
            let details =
                Paragraph::new("Select an operation for details...").alignment(Alignment::Center);
            f.render_widget(details, details_table_chunk);
            return;
        };

        let table = Table::new(rows)
            .header(details_table_header)
            .block(details_table_block)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
            ]);

        f.render_stateful_widget(
            table,
            details_table_chunk,
            &mut ui_state.details_operation_statistics_table_state,
        );

        let elapsed = now.elapsed();
        info!(log, "Render time: {}ms", elapsed.as_millis())
    }
}
