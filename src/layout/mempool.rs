use conv::ValueFrom;

use tui::style::Modifier;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::ui::Ui;

pub struct MempoolScreen {}

impl MempoolScreen {
    pub fn draw_mempool_screen<B: Backend>(ui: &mut Ui, f: &mut Frame<B>) {
        let size = f.size();

        // TODO: placeholder for mempool page
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(size);

        // dummy
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        // ======================== PAGES TABS ========================
        let tabs = ui.create_pages_tabs();
        f.render_widget(tabs, chunks[1]);
    }
}