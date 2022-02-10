use std::io::Stdout;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::Frame;

use crate::automaton::State;
use crate::common::{create_header_bar, create_pages_tabs, create_quit};
use crate::extensions::Renderable;

use super::{BlockApplicationSummary};

// TODO: will this be the actual homescreen?
pub struct BakingScreen {}

impl Renderable for BakingScreen {
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

        let (baking_table_chunk, summary_chunk) = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(page_chunks[2])
            .into_iter()
            .collect_tuple()
            .unwrap();

        // ======================== SUMMARY PANEL (right) ========================
        let selected_style = Style::default().remove_modifier(Modifier::DIM);
        let normal_style = Style::default().fg(Color::White);

        // TODO - panic: handle this vector, shold be a vector in the first place? With new architecture that ignores reorgs?
        let application_summary =
            BlockApplicationSummary::from(state.baking.application_statistics[0].clone());
        let application_stats_table_data = application_summary.to_table_data();

        let headers = vec!["OPERATION", "DURATION"];
        let header_cells = headers.iter().map(|header| Cell::from(*header));

        let rows = application_stats_table_data.into_iter().map(|stat| {
            let height = 1;

            let tag = Cell::from(stat.0);
            let value = Cell::from(stat.1.to_string());
            Row::new(vec![tag, value]).height(height)
        });
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let block = Block::default().borders(Borders::ALL);
        let table = Table::new(rows)
            .header(header)
            .block(block)
            .widths(&[Constraint::Percentage(75), Constraint::Percentage(25)]);
        f.render_widget(table, summary_chunk);

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
        let header = &state.current_head_header;
        create_header_bar(page_chunks[0], header, f);

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(&state.ui);
        f.render_widget(tabs, page_chunks[3]);

        // ======================== Quit ========================
        create_quit(page_chunks[3], f);
    }
}
