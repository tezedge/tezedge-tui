use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::Corner;
use tui::style::Modifier;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use itertools::Itertools;

use crate::automaton::State;
use crate::common::{create_header_bar, create_help_bar, create_pages_tabs, create_quit};
use crate::extensions::{CustomSeparator, Renderable};

const SIDE_PADDINGS: u16 = 1;
const SIDE_BY_SIDE_TABLE_THRESHOLD: u16 = 128;

pub struct StatisticsScreen {}

impl Renderable for StatisticsScreen {
    fn draw_screen(state: &State, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let size = f.size();
        let delta_toggle = state.delta_toggle;

        // TODO: maybe we just leave this on default
        // set the bacground color
        let background = Block::default().style(Style::default().bg(Color::Rgb(31, 30, 30)));
        f.render_widget(background, size);

        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(size);

        // ======================== HEADER ========================
        let header = &state.current_head_header;
        create_header_bar(page_chunks[0], header, f);

        let operations_statistics = &state.operations_statistics.operations_statistics;
        let operations_statistics_sortable = &state
            .operations_statistics
            .main_operation_statistics_table
            .content;

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(&state.ui);
        f.render_widget(tabs, page_chunks[3]);

        // ======================== Quit ========================
        create_quit(page_chunks[3], f);

        // Display a loading data screen until the data is loaded
        if operations_statistics.is_empty() {
            let loading = Paragraph::new("Loading data...").alignment(Alignment::Center);
            f.render_widget(loading, page_chunks[1]);
            return;
        }

        // ======================== HELP BAR ========================
        create_help_bar(page_chunks[1], f, delta_toggle);

        let (main_table_chunk, details_table_chunk) =
            if f.size().width < SIDE_BY_SIDE_TABLE_THRESHOLD {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(75), Constraint::Length(25)])
                    .split(page_chunks[2])
                    .into_iter()
                    .collect_tuple()
                    .unwrap()
            } else {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(page_chunks[2])
                    .into_iter()
                    .collect_tuple()
                    .unwrap()
            };

        // ======================== MAIN STATISTICS TABLE ========================
        let main_table_block = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().remove_modifier(Modifier::DIM);
        let normal_style = Style::default().fg(Color::White);

        let max_size: u16 = if f.size().width < SIDE_BY_SIDE_TABLE_THRESHOLD {
            f.size().width - SIDE_PADDINGS
        } else {
            f.size().width - details_table_chunk.width - SIDE_PADDINGS
        };

        let renderable_constraints = state
            .operations_statistics
            .main_operation_statistics_table
            .renderable_constraints(max_size);

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

        // overlap the block corners with special separators to make flush transition to the table block
        let vertical_left_separator = CustomSeparator::default()
            .separator("├")
            .corner(Corner::TopLeft);
        f.render_widget(vertical_left_separator, main_table_chunk);
        let horizontal_down_separator = CustomSeparator::default()
            .separator("┬")
            .corner(Corner::TopRight);
        f.render_widget(horizontal_down_separator, main_table_chunk);
        let horizontal_up_separator = CustomSeparator::default()
            .separator("┴")
            .corner(Corner::BottomRight);
        f.render_widget(horizontal_up_separator, main_table_chunk);

        // ======================== DETAILS TABLE ========================
        let details_table_block =
            Block::default().borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM);

        let renderable_constraints = state
            .operations_statistics
            .details_operation_statistics_table
            .renderable_constraints(details_table_chunk.width);

        let header_cells = state
            .operations_statistics
            .details_operation_statistics_table
            .renderable_headers(selected_style);
        let details_table_header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let table_content = &state
            .operations_statistics
            .details_operation_statistics_table
            .content;

        let rows = if !table_content.is_empty() {
            state
                .operations_statistics
                .details_operation_statistics_table
                .renderable_rows(table_content, delta_toggle)
        } else {
            let details = Paragraph::new("Select an operation for details...")
                .alignment(Alignment::Center)
                .block(details_table_block);
            f.render_widget(details, details_table_chunk);
            let vertical_right_separator = CustomSeparator::default()
                .separator("┤")
                .corner(Corner::TopRight);
            f.render_widget(vertical_right_separator, details_table_chunk);
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

        let vertical_right_separator = CustomSeparator::default()
            .separator("┤")
            .corner(Corner::TopRight);
        f.render_widget(vertical_right_separator, details_table_chunk);
    }
}
