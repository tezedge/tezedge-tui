use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{Frame, Terminal, backend::{Backend, CrosstermBackend}, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Span, Spans}, widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table}};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let block = Block::default().borders(Borders::NONE);
    f.render_widget(block, size);

    // split the screen to 3 parts vertically
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(5), Constraint::Min(2), Constraint::Length(10)].as_ref())
        .split(f.size());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // ======================== HEADER AND OPs (TOP LEFT RECT) ========================
    let headers_and_operations_block = Block::default()
        .title("Syncing headers and operations")
        .borders(Borders::ALL);
    // f.render_widget(headers_and_operations_block, top_chunks[0]);

    // TODO: replace mocked data
    let paragraph = Paragraph::new(vec![
        Spans::from("59%  ETA  18m 50s"),
        Spans::from("569999 level"),
        Spans::from("16 blocks / s"),
    ])
    .style(Style::default())
    .block(headers_and_operations_block)
    .alignment(Alignment::Left);
    f.render_widget(paragraph, top_chunks[0]);

    // ======================== APPLYING (TOP RIGHT RECT) ========================
    let applying = Block::default()
        .title("Applying Operations")
        .borders(Borders::ALL);
    // f.render_widget(applying, top_chunks[1]);

    let paragraph = Paragraph::new(vec![
        Spans::from("2%  ETA  1d 18h 18m 50s"),
        Spans::from("2231 level"),
        Spans::from("3 blocks / s"),
    ])
    .style(Style::default())
    .block(applying)
    .alignment(Alignment::Left);
    f.render_widget(paragraph, top_chunks[1]);

    // ======================== CHAIN STATUS ========================
    let chain_status = Block::default()
        .borders(Borders::ALL);
    // f.render_widget(chain_status, chunks[1]);

    let periods_left_right = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // // devide the chain status into further chunks
    // let period_heigth = 4;
    // let period_width = 32;   // TODO: this changes in granada!

    // let vertical_count = period_chunks[0].height / period_heigth;
    // let horizontal_count = period_chunks[0].width / period_width;
    // let mut grid: Vec<Vec<Rect>> = Vec::with_capacity((vertical_count * horizontal_count).into());

    // let mut period_grid_vertical: Vec<Constraint> = Vec::with_capacity(vertical_count as usize);
    // for _ in 0..vertical_count {
    //     period_grid_vertical.push(Constraint::Max(period_heigth))
    // }

    // let mut period_grid_horizontal: Vec<Constraint> = Vec::with_capacity(horizontal_count as usize);
    // for _ in 0..horizontal_count {
    //     period_grid_horizontal.push(Constraint::Max(period_width))
    // }

    // let period_rows = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints(period_grid_vertical)
    //     .split(period_chunks[0]);
    // for period_row in period_rows {
    //     let periods = Layout::default()
    //         .direction(Direction::Horizontal)
    //         .constraints(period_grid_horizontal.clone())
    //         .split(period_row);
    //     grid.push(periods)
    // }

    // let cycles_per_period = [Constraint::Length(4); 8];

    // for row in grid {
    //     for column in row {
    //         let dummy_block = Block::default()
    //             .borders(Borders::ALL);
    //         // f.render_widget(dummy_block, column);
    //         let column_chunk = Layout::default()
    //             .direction(Direction::Vertical)
    //             .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    //             .split(column);
    //         let period_name = Paragraph::new(Spans::from("Unknown")).alignment(Alignment::Left).block(Block::default());
    //         f.render_widget(period_name, column_chunk[0]);

    //         let cycles = Layout::default()
    //             .direction(Direction::Horizontal)
    //             .constraints(cycles_per_period).margin(0)
    //             .split(column_chunk[1]);

    //         for cycle in cycles {
    //             let dummy_block = Block::default()
    //                 .borders(Borders::ALL);
    //             f.render_widget(dummy_block, cycle);
    //         }
    //     }
    // }
    let cycle_block_width = 5;
    let cycle_block_heigth = 3;
    let period_block_width = 42;
    let period_block_height = 8;

    for container in periods_left_right {
        let periods_container = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min((container.width - period_block_width) / 2),
            Constraint::Length(period_block_width),
            Constraint::Min((container.width - period_block_width) / 2),
        ])
        .split(container);
    
        // test render
        // let dummy_block = Block::default().borders(Borders::ALL);
        // f.render_widget(dummy_block, periods_container[1]);

        let period_count_per_page = periods_container[1].height / period_block_height;
        let vertical_padding = (periods_container[1].height - (period_count_per_page * period_block_height)) / 2;

        let row_constraints = std::iter::repeat(Constraint::Length(period_block_height))
            .take(period_count_per_page.into())
            .collect::<Vec<_>>();

        let column_constraints = std::iter::repeat(Constraint::Length(5))
            .take(8)
            .collect::<Vec<_>>();

        let periods_container_exact = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(vertical_padding),
                Constraint::Length(period_count_per_page * period_block_height),
                Constraint::Min(vertical_padding),
            ])
            .split(periods_container[1]);
        
        // test render
        // let dummy_block = Block::default().borders(Borders::ALL);
        // f.render_widget(dummy_block, periods_container_exact[1]);

        let periods = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(periods_container_exact[1]);

        // test render
        for period in periods {
            let period_chunks = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(period);
            
            let period_name = Paragraph::new(Spans::from(" Proposal")).alignment(Alignment::Left).block(Block::default());
            f.render_widget(period_name, period_chunks[0]);

            let cycles = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(column_constraints.clone())
                .horizontal_margin(1)
                .vertical_margin(0)
                .split(period_chunks[1]);
            
            for cycle in cycles {
                let pad_line = " ".repeat(cycle_block_width);
                let inside_block_line = " ".repeat(cycle_block_width - 2);
                let pad_line_num = cycle_block_heigth - 3;  // TODO
                let text = std::iter::repeat(pad_line.clone())
                    .take(pad_line_num / 2)
                    .chain(std::iter::once(inside_block_line.clone()))
                    .chain(std::iter::repeat(pad_line).take(pad_line_num / 2))
                    .collect::<Vec<_>>()
                    .join("\n");
                let cycle_block_text = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).style(Style::default().bg(Color::Black).fg(Color::White)))
                    .alignment(Alignment::Center)
                    .style(Style::default().bg(Color::Cyan).fg(Color::Black));
                // render cycle blocks
                // f.render_widget(cycle_block, cycle);
                // render the "text" (padded background)
                f.render_widget(cycle_block_text, cycle);
            }
            
            // render outer borders
            f.render_widget(Block::default().borders(Borders::ALL), period);
        }
    }

    
    // ======================== CONNECTED PEERS ========================
    let connected_peers = Block::default()
        .borders(Borders::ALL);
    // table
    let items = vec![
        vec!["88.213.174.203:9732", "468.23 kB", "2.61 KB/s", "2.61 KB/s"],
        vec!["138.201.74.178:9733", "12.01 kB", "91 B/s", "91 B/s"],
        vec!["66.70.178.32:9732", "282.64 kB", "0 B/s", "0 B/s"],
        vec!["162.55.163.248:9732", "64.35 kB", "0 B/s", "0 B/s"],
        vec!["88.213.174.203:9732", "468.23 kB", "2.61 KB/s", "2.61 KB/s"],
        vec!["88.213.174.203:9732", "468.23 kB", "2.61 KB/s", "2.61 KB/s"],
        vec!["88.213.174.203:9732", "468.23 kB", "2.61 KB/s", "2.61 KB/s"],
    ];

    let header_cells = ["Address", "Total", "Average", "Current"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default()));
    let header = Row::new(header_cells).height(1).bottom_margin(1);
    let rows = items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16)
    });
    let table = Table::new(rows)
        .header(header)
        .block(connected_peers)
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25)
        ]);
    f.render_widget(table, chunks[2]);
    
}

pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
