use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::style::Modifier;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use itertools::Itertools;

use crate::automaton::State;
use crate::common::{create_header_bar, create_help_bar, create_pages_tabs};
use crate::extensions::Renderable;

const SIDE_PADDINGS: u16 = 1;
const SIDE_BY_SIDE_TABLE_THRESHOLD: u16 = 128;

pub struct StatisticsScreen {}

impl Renderable for StatisticsScreen {
    fn draw_screen(state: &State, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let size = f.size();

        let delta_toggle = state.delta_toggle;

        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(5),
                Constraint::Length(3),
                Constraint::Length(4),
            ])
            .split(size);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(&state.ui);
        f.render_widget(tabs, page_chunks[2]);

        let operations_statistics = &state.operations_statistics.operations_statistics;
        let operations_statistics_sortable =
            &state.operations_statistics.operations_statistics_sortable;

        // Display a loading data screen until the data is loaded
        if operations_statistics.is_empty() {
            let loading = Paragraph::new("Loading data...").alignment(Alignment::Center);
            f.render_widget(loading, size);
            return;
        }

        let (main_table_chunk, details_table_chunk) =
            if f.size().width < SIDE_BY_SIDE_TABLE_THRESHOLD {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(75), Constraint::Length(25)])
                    .split(page_chunks[1])
                    .into_iter()
                    .collect_tuple()
                    .unwrap()
            } else {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Min(64), Constraint::Length(64)])
                    .split(page_chunks[1])
                    .into_iter()
                    .collect_tuple()
                    .unwrap()
            };

        // ======================== HELP BAR ========================
        create_help_bar(page_chunks[3], f, delta_toggle);

        // ======================== HEADER ========================
        let header = &state.current_head_header;
        create_header_bar(page_chunks[0], header, f);

        // ======================== MAIN STATISTICS TABLE ========================
        let main_table_block = Block::default().borders(Borders::ALL).title("Operations");

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let max_size: u16 = if f.size().width < SIDE_BY_SIDE_TABLE_THRESHOLD {
            f.size().width - SIDE_PADDINGS
        } else {
            f.size().width - details_table_chunk.width - SIDE_PADDINGS
        };

        let renderable_constraints = state
            .operations_statistics
            .main_operation_statistics_table
            .renderable_constraints(max_size);

        // state.operations_statistics.main_operation_statistics_table.highlight_sorting();
        let header_cells = state
            .operations_statistics
            .main_operation_statistics_table
            .renderable_headers(selected_style);

        let main_table_header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = state
            .operations_statistics
            .main_operation_statistics_table
            .renderable_rows(operations_statistics_sortable, delta_toggle);

        let table = Table::new(rows)
            .header(main_table_header)
            .block(main_table_block)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&renderable_constraints);

        f.render_stateful_widget(
            table,
            main_table_chunk,
            &mut state
                .operations_statistics
                .main_operation_statistics_table
                .table_state
                .clone(),
        );

        // ======================== DETAILS TABLE ========================

        let details_table_block = Block::default().borders(Borders::ALL).title("Details");

        let renderable_constraints = state
            .operations_statistics
            .details_operation_statistics_table
            .renderable_constraints(details_table_chunk.width);

        // state.operations_statistics
        //     .details_operation_statistics_table
        //     .highlight_sorting();

        let header_cells = state
            .operations_statistics
            .details_operation_statistics_table
            .renderable_headers(selected_style);
        let details_table_header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = if let Some(details) = &state.operations_statistics.selected_operation_details {
            state
                .operations_statistics
                .details_operation_statistics_table
                .renderable_rows(details, delta_toggle)
        } else {
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
            .widths(&renderable_constraints);

        f.render_stateful_widget(
            table,
            details_table_chunk,
            &mut state
                .operations_statistics
                .details_operation_statistics_table
                .table_state
                .clone(),
        );
    }
}
