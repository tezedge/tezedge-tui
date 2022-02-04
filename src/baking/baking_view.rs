use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{BarChart, Block, Borders};
use tui::Frame;

use crate::automaton::State;
use crate::common::{create_header_bar, create_pages_tabs, create_quit};
use crate::extensions::Renderable;

use super::ToHistogramData;

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

        let histogram_data = state.baking.per_peer_block_statistics.to_histogram_data();

        let histogram: Vec<(&str, u64)> = histogram_data
            .iter()
            .map(|(label, value)| (&**label, *value))
            .collect();

        let barchart = BarChart::default()
            .block(
                Block::default()
                    .title("RECEIVED TIME COUNT HISTOGRAM")
                    .borders(Borders::ALL),
            )
            .data(histogram.as_slice())
            .bar_width(8)
            .value_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .bar_style(Style::default().fg(Color::Cyan));

        // let test = Paragraph::new(format!("{:?}", histogram));
        f.render_widget(barchart, page_chunks[2]);

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
