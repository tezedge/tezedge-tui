use conv::{ConvUtil, ValueFrom};
use std::convert::TryFrom;
use std::io::SeekFrom;
use std::ops::Add;
use std::{convert::TryInto, io};

use crossterm::event::{self, Event, KeyCode};
use tokio::sync::mpsc;
use tokio::time::Duration;
use tui::style::Modifier;
use tui::widgets::TableState;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};

use crate::model::{RollingList, StateRef, UiState};
#[derive(Default)]
pub struct Ui {
    pub state: StateRef,
    pub ui_state: UiState,
}

impl Ui {
    fn syncing_screen<B: Backend>(&mut self, f: &mut Frame<B>) {
        let state = self.state.read().unwrap();
        let page_in_focus = self.ui_state.pages.in_focus();
        let widget_in_focus = self.ui_state.pages.widget_state[page_in_focus].in_focus();

        let size = f.size();

        let block = Block::default().borders(Borders::NONE);
        f.render_widget(block, size);

        // split the screen to 3 parts vertically
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(5),
                    Constraint::Min(2),
                    Constraint::Length(10),
                ]
                .as_ref(),
            )
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

        let syncing_eta = if let Some(eta) = state.incoming_transfer.eta {
            eta
        } else {
            0.0
        };

        let paragraph = Paragraph::new(vec![
            Spans::from(format!(
                "{:.2}% {}",
                calculate_percentage(
                    state.incoming_transfer.current_block_count,
                    state.incoming_transfer.downloaded_blocks
                ),
                convert_eta(syncing_eta)
            )),
            Spans::from(format!(
                "{} level",
                state.incoming_transfer.downloaded_blocks
            )),
            Spans::from(format!(
                "{:.2} blocks / s",
                state.incoming_transfer.download_rate
            )),
        ])
        .style(Style::default())
        .block(headers_and_operations_block)
        .alignment(Alignment::Left);
        f.render_widget(paragraph, top_chunks[0]);
        // f.render_stateful_widget(paragraph, top_chunks[0], )

        // ======================== APPLYING (TOP RIGHT RECT) ========================
        let applying = Block::default()
            .title("Applying Operations")
            .borders(Borders::ALL);
        // f.render_widget(applying, top_chunks[1]);

        let application_eta = if state.aplication_status.current_application_speed != 0.0 {
            (state.incoming_transfer.current_block_count - state.last_applied_level as usize) as f32
                / state.aplication_status.current_application_speed
                * 60.0
        } else {
            0.0
        };

        let paragraph = Paragraph::new(vec![
            Spans::from(format!(
                "{:.2}% {}",
                calculate_percentage(
                    state.incoming_transfer.current_block_count,
                    state.last_applied_level as usize
                ),
                convert_eta(application_eta)
            )),
            Spans::from(format!("{} level", state.last_applied_level)),
            Spans::from(format!(
                "{:.2} blocks / s",
                state.aplication_status.current_application_speed / 60.0
            )),
        ])
        .style(Style::default())
        .block(applying)
        .alignment(Alignment::Left);
        f.render_widget(paragraph, top_chunks[1]);

        // ======================== CHAIN STATUS ========================
        let chain_status = Block::default().borders(Borders::ALL);
        // f.render_widget(chain_status, chunks[1]);

        let cycle_block_width = 5;
        let cycle_block_heigth = 3;
        let period_block_width = 42;
        let period_block_height = cycle_block_heigth + 3;

        let cycle_per_period = 8;
        let period_count_per_page_on_heigth = chunks[1].height / period_block_height;
        let period_count_per_page_on_width = chunks[1].width / period_block_width;

        let vertical_padding =
            (chunks[1].height - (period_count_per_page_on_heigth * period_block_height)) / 2;
        let horizontal_padding =
            (chunks[1].width - (period_count_per_page_on_width * period_block_width)) / 2;

        let cycle_count = state.cycle_data.len();
        let period_count = cycle_count / cycle_per_period;
        // let period_count = divide_round_up(cycle_count, cycle_per_period);

        // selected period container state
        self.ui_state.period_info_state.displayable_container_count =
            period_count_per_page_on_heigth.into();
        self.ui_state.period_info_state.container_count =
            period_count / period_count_per_page_on_width as usize;
        // self.ui_state.period_info_state.container_count = divide_round_up(period_count, period_count_per_page_on_width as usize);

        // if let Some(selected_container) = self.ui_state.period_info_state.selected {
        //     if selected_container >= self.ui_state.period_info_state.displayable_container_count {
        //         self.ui_state.period_info_state.selected = Some(self.ui_state.period_info_state.displayable_container_count - 1)
        //     }
        // }

        let period_containers_row_constraints =
            std::iter::repeat(Constraint::Length(period_block_height))
                .take(period_count_per_page_on_heigth.into())
                .collect::<Vec<_>>();

        let periods_containers_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(vertical_padding),
                Constraint::Length(period_block_height * period_count_per_page_on_heigth),
                Constraint::Min(vertical_padding),
            ])
            .split(chunks[1])[1];

        let periods_containers = Layout::default()
            .direction(Direction::Vertical)
            .constraints(period_containers_row_constraints)
            .split(periods_containers_chunk);

        let applied_style = Style::default().bg(Color::Cyan).fg(Color::Black);
        let dowloaded_style = Style::default().bg(Color::Gray).fg(Color::Black);
        let default_style = Style::default().bg(Color::Black).fg(Color::Black);

        // TODO: more appropriate approach
        // do not render while we have no data
        if state.block_metrics.is_empty() {
            return;
        }

        for (container_index, container) in periods_containers.into_iter().enumerate() {
            let periods_container = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(period_block_width * period_count_per_page_on_width),
                    Constraint::Min(horizontal_padding * 2),
                ])
                .split(container)[0];

            if let Some(selected_container) = self.ui_state.period_info_state.selected {
                if widget_in_focus == 0
                    && selected_container
                        == container_index + self.ui_state.period_info_state.offset()
                {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Blue));
                    f.render_widget(block, container);
                }
            }

            // test render
            // let dummy_block = Block::default().borders(Borders::ALL);
            // f.render_widget(dummy_block, periods_container);

            let row_constraints = std::iter::repeat(Constraint::Length(period_block_width))
                .take(period_count_per_page_on_width.into())
                .collect::<Vec<_>>();

            let column_constraints = std::iter::repeat(Constraint::Length(5))
                .take(cycle_per_period)
                .collect::<Vec<_>>();

            let periods = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(row_constraints)
                .split(periods_container);

            for (period_index, period) in periods.into_iter().enumerate() {
                // only render periods that are present on the netrwork
                if (container_index + self.ui_state.period_info_state.offset())
                    * period_count_per_page_on_width as usize
                    + period_index
                    > period_count
                {
                    break;
                }
                let period_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .vertical_margin(1)
                    .constraints([Constraint::Length(1), Constraint::Length(3)])
                    .split(period);

                let period_name = Paragraph::new(Spans::from(" Proposal"))
                    .alignment(Alignment::Left)
                    .block(Block::default());
                f.render_widget(period_name, period_chunks[0]);

                let cycles = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(column_constraints.clone())
                    .horizontal_margin(1)
                    .vertical_margin(0)
                    .split(period_chunks[1]);

                for (cycle_index, cycle) in cycles.into_iter().enumerate() {
                    let cycle_data_index = ((container_index
                        + self.ui_state.period_info_state.offset())
                        * period_count_per_page_on_width as usize
                        * cycle_per_period)
                        + (period_index * cycle_per_period)
                        + cycle_index;

                    // do not render cycles that are not present on the chain
                    if cycle_data_index > cycle_count {
                        break;
                    }

                    let pad_line = " ".repeat(cycle_block_width);
                    // let inside_block_line = " ".repeat(cycle_block_width - 2);
                    let inside_block_line = format!(
                        "{:^length$}",
                        cycle_data_index,
                        length = (cycle_block_width - 2)
                    );
                    let pad_line_num = cycle_block_heigth - 3; // TODO
                    let text = std::iter::repeat(pad_line.clone())
                        .take((pad_line_num / 2).into())
                        .chain(std::iter::once(inside_block_line.clone()))
                        .chain(std::iter::repeat(pad_line).take((pad_line_num / 2).into()))
                        .collect::<Vec<_>>()
                        .join("\n");
                    let cycle_block_text = Paragraph::new(text)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .style(Style::default().bg(Color::Black).fg(Color::White)),
                        )
                        .alignment(Alignment::Center)
                        .style(if state.block_metrics.len() <= cycle_data_index {
                            default_style
                        } else if state.block_metrics[cycle_data_index].all_applied() {
                            applied_style
                        } else if state.block_metrics[cycle_data_index].all_downloaded() {
                            dowloaded_style
                        } else {
                            default_style
                        });
                    // render the "text" (padded background)
                    f.render_widget(cycle_block_text, cycle);
                }

                // render outer borders
                // f.render_widget(Block::default().borders(Borders::ALL), period);
            }
        }

        // ======================== CONNECTED PEERS ========================
        let connected_peers =
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if widget_in_focus == 1 {
                    Color::Blue
                } else {
                    Color::White
                }));
        // table
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = ["Address", "Total", "Average", "Current"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default()));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        let rows = state.peer_metrics.iter().map(|item| {
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
            .block(connected_peers)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]);
        if self.ui_state.pages.index == 0 && widget_in_focus == 1 {
            f.render_stateful_widget(table, chunks[2], &mut self.ui_state.peer_table_state);
        } else {
            f.render_widget(table, chunks[2]);
        }
    }

    pub async fn run_tui<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut events = events(tick_rate);
        loop {
            // Note: here we decide what screen to draw
            terminal.draw(|f| self.syncing_screen(f))?;

            match events.recv().await {
                Some(TuiEvent::Input(key)) => match key {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => self.next(),
                    KeyCode::Up => self.previous(),
                    KeyCode::Right => self.ui_state.pages.next(),
                    KeyCode::Left => self.ui_state.pages.previous(),
                    KeyCode::Tab => {
                        self.ui_state.pages.widget_state[self.ui_state.pages.index].next()
                    }
                    _ => {}
                },
                Some(TuiEvent::Tick) => {}
                None => return Ok(()),
                _ => {}
            }
        }
    }

    pub fn next(&mut self) {
        let page_in_focus = self.ui_state.pages.in_focus();
        match page_in_focus {
            0 => {
                let widget_in_focus = self.ui_state.pages.widget_state[page_in_focus].in_focus();
                match widget_in_focus {
                    1 => {
                        let state = self.state.read().unwrap();
                        if state.peer_metrics.is_empty() {
                            return;
                        }

                        let i = match self.ui_state.peer_table_state.selected() {
                            Some(i) => {
                                if i >= state.peer_metrics.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        self.ui_state.peer_table_state.select(Some(i));
                    }
                    0 => {
                        let to_select = match self.ui_state.period_info_state.selected() {
                            Some(to_select) => {
                                if to_select >= self.ui_state.period_info_state.container_count - 1
                                {
                                    0
                                } else {
                                    to_select + 1
                                }
                            }
                            None => 0,
                        };
                        self.ui_state.period_info_state.select(Some(to_select));
                    }
                    _ => {}
                }
            }
            1 => {
                todo!()
            }
            _ => {}
        }
    }

    pub fn previous(&mut self) {
        let page_in_focus = self.ui_state.pages.in_focus();
        match page_in_focus {
            0 => {
                let widget_in_focus = self.ui_state.pages.widget_state[page_in_focus].in_focus();
                match widget_in_focus {
                    1 => {
                        let state = self.state.read().unwrap();
                        if state.peer_metrics.is_empty() {
                            return;
                        }

                        let i = match self.ui_state.peer_table_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    state.peer_metrics.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        self.ui_state.peer_table_state.select(Some(i));
                    }
                    0 => {
                        let to_select = match self.ui_state.period_info_state.selected() {
                            Some(to_select) => {
                                if to_select == 0 {
                                    self.ui_state.period_info_state.container_count - 1
                                } else {
                                    to_select - 1
                                }
                            }
                            None => 0,
                        };
                        self.ui_state.period_info_state.select(Some(to_select));
                    }
                    _ => {}
                }
            }
            1 => {
                todo!()
            }
            _ => {}
        }
    }
}

enum TuiEvent {
    Input(KeyCode),
    Resize,
    Mouse,
    Tick,
}

fn events(tick_rate: Duration) -> mpsc::Receiver<TuiEvent> {
    let (tx, rx) = mpsc::channel(100);
    let keys_tx = tx.clone();

    tokio::spawn(async move {
        loop {
            match event::read() {
                Ok(Event::Key(key)) => {
                    if let Err(err) = keys_tx.send(TuiEvent::Input(key.code)).await {
                        eprintln!("{}", err);
                        break;
                    }
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
                Ok(Event::Resize(_, _)) => {
                    if let Err(err) = keys_tx.send(TuiEvent::Resize).await {
                        eprintln!("{}", err);
                        break;
                    }
                }
                Ok(Event::Mouse(_)) => {
                    if let Err(err) = keys_tx.send(TuiEvent::Mouse).await {
                        eprintln!("{}", err);
                        break;
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            if let Err(err) = tx.send(TuiEvent::Tick).await {
                eprintln!("{}", err);
                break;
            }
            tokio::time::sleep(tick_rate).await;
        }
    });

    rx
}

fn convert_eta(eta: f32) -> String {
    let days = (eta / 86400.0).floor();
    let hours = ((eta / 3600.0) % 24.0).floor();
    let minutes = ((eta / 60.0) % 60.0).floor();
    let seconds = (eta % 60.0).floor();

    format!("ETA {}d {}h {}m {}s", days, hours, minutes, seconds)
}

fn calculate_percentage(total: usize, current: usize) -> f32 {
    if total == 0 {
        return 0.0;
    }

    let total = f32::value_from(total).unwrap();
    let current = f32::value_from(current).unwrap();

    current / total * 100.0
}

fn divide_round_up(dividend: usize, divisor: usize) -> usize {
    (dividend + (divisor - 1)) / divisor
}
