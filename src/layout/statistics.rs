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

use crate::model::{StateRef, UiState};

use super::create_pages_tabs;
pub struct StatisticsScreen {}

impl StatisticsScreen {
    pub fn draw_statistics_screen<B: Backend>(
        data_state: &StateRef,
        ui_state: &mut UiState,
        f: &mut Frame<B>,
    ) {
        let size = f.size();

        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(5), Constraint::Length(3)])
            .split(size);

        let (main_table_chunk, details_table_chunk) = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Max(120), Constraint::Min(3)])
            .split(page_chunks[0])
            .into_iter()
            .collect_tuple()
            .unwrap();

        // ======================== MAIN STATISTICS TABLE ========================
        let main_table_headers: Vec<String> = [
            "Datetime", "Hash", "Nodes", "Delta", "Received", "Con.Rec.", "Valid.S.", "Preap.S.",
            "Preap.F.", "Valid.F.", "Val.Len.", "Sent", "Kind",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();

        let main_table_block = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = main_table_headers
            .iter()
            .map(|h| Cell::from(h.as_str()).style(Style::default()));
        let main_table_header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        // TODO: replace mocked data
        let rows =
            std::iter::repeat(Row::new(std::iter::repeat(Cell::from("MOCK")).take(13))).take(15);

        let table = Table::new(rows)
            .header(main_table_header)
            .block(main_table_block)
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
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
            ]);

        f.render_widget(table, main_table_chunk);

        // ======================== DETAILS TABLE ========================

        let main_table_headers: Vec<String> = [
            "Node Id", "1.Rec.", "1.Rec.C.", "1.Sent", "Received", "Con.Rec.", "Sent",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();

        let details_table_block = Block::default().borders(Borders::ALL);

        let header_cells = main_table_headers
            .iter()
            .map(|h| Cell::from(h.as_str()).style(Style::default()));
        let details_table_header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        // TODO: replace mocked data
        let rows =
            std::iter::repeat(Row::new(std::iter::repeat(Cell::from("D.MOCK")).take(7))).take(12);

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

        f.render_widget(table, details_table_chunk);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(ui_state);
        f.render_widget(tabs, page_chunks[1]);
    }
}
