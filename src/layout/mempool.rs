use conv::ValueFrom;

use tui::layout::Rect;
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

use crate::model::CurrentHeadHeader;
use crate::node_rpc::{RpcCall, RpcResponse};
use crate::ui::Ui;
pub struct MempoolScreen {}

impl MempoolScreen {
    pub fn draw_mempool_screen<B: Backend>(ui: &mut Ui, f: &mut Frame<B>) {
        let state = ui.state.read().unwrap();
        let size = f.size();

        // TODO: placeholder for mempool page
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(5), Constraint::Length(3)])
            .split(size);

        let (header_chunk, summary_chunk, endorsements_chunk) = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(4),
                Constraint::Min(1),
            ])
            .split(chunks[0])
            .into_iter()
            .collect_tuple()
            .unwrap(); // safe as we specify 3 elements in constraints and collecting into tuple of size 3

        // ======================== HEADER ========================
        // wrap the header chunk in border
        let block = Block::default().borders(Borders::ALL).title("Header");
        f.render_widget(block, header_chunk);

        let header = &state.current_head_header;

        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
            .split(header_chunk);

        let block_hash = Paragraph::new(Spans::from(format!("Current head hash: {}", header.hash)))
            .block(Block::default())
            .alignment(Alignment::Left);
        f.render_widget(block_hash, header_chunks[0]);

        let block_level = Paragraph::new(format!("Current head level: {}", header.level))
            .block(Block::default())
            .alignment(Alignment::Left);
        f.render_widget(block_level, header_chunks[1]);

        let block_protocol = Paragraph::new(format!("Protocol: {}", header.protocol))
            .block(Block::default())
            .alignment(Alignment::Left);
        f.render_widget(block_protocol, header_chunks[2]);

        // ======================== SUMARY ========================
        let summary_elements_constraits = std::iter::repeat(Constraint::Percentage(16))
            .take(6)
            .collect::<Vec<_>>();

        let endorsement_statuses: Vec<String> = vec![
            "Missing",
            "Broadcasted",
            "Applied",
            "Prechecked",
            "Decoded",
            "Receives",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();

        // TODO: replace mocked data
        let endorsement_statuses_values = vec![123, 11, 31, 33, 12, 22];

        let sumary_blocks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(summary_elements_constraits)
            .split(summary_chunk);

        for (i, sumary_block) in sumary_blocks.into_iter().enumerate() {
            let block_text = Paragraph::new(format!(
                "{}\n{}",
                endorsement_statuses[i], endorsement_statuses_values[i]
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black).fg(Color::White)),
            )
            .alignment(Alignment::Center);
            f.render_widget(block_text, sumary_block)
        }

        // ======================== ENDORSERS ========================
        let endorsers = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        // TODO: add all the columns!
        let header_cells = [
            "Slots",
            "Baker",
            "Status",
            "Delta",
            "Receive",
            "Decode",
            "Precheck",
            "Apply",
            "Broadcast",
        ]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default()));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = state.current_head_endorsement_rights.iter().map(|item| {
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
            .header(header)
            .block(endorsers)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Length(4),
                Constraint::Length(36),
                Constraint::Min(11),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(8),
                Constraint::Min(9),
            ]);
        f.render_widget(table, endorsements_chunk);

        // dummy
        // let block = Block::default().borders(Borders::ALL).title("Endorsements");
        // f.render_widget(block, chunks[0]);

        // ======================== PAGES TABS ========================
        let tabs = ui.create_pages_tabs();
        f.render_widget(tabs, chunks[1]);
    }
}
