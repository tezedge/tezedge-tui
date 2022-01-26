use std::io::Stdout;

use conv::ValueFrom;

use tui::backend::CrosstermBackend;
use tui::style::Modifier;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::automaton::State;
use crate::common::{create_help_bar, create_pages_tabs};
use crate::extensions::Renderable;
use crate::terminal_ui::ActiveWidget;

pub struct SynchronizationScreen {}

impl Renderable for SynchronizationScreen {
    fn draw_screen(state: &State, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let widget_in_focus = &state.ui.active_widget;
        let delta_toggle = state.delta_toggle;

        let size = f.size();

        let block = Block::default().borders(Borders::NONE);
        f.render_widget(block, size);

        // split the screen to 4 parts vertically
        let page_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(5),
                    Constraint::Min(2),
                    Constraint::Length(10),
                    Constraint::Length(3),
                    Constraint::Length(4),
                ]
                .as_ref(),
            )
            .split(f.size());

        // ======================== HEADER AND OPs (TOP LEFT RECT) ========================
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(page_chunks[0]);

        let headers_and_operations_block = Block::default()
            .title("Syncing headers and operations")
            .borders(Borders::ALL);
        // f.render_widget(headers_and_operations_block, top_chunks[0]);

        let syncing_eta = if let Some(eta) = state.synchronization.incoming_transfer.eta {
            eta
        } else {
            0.0
        };

        let paragraph = Paragraph::new(vec![
            Spans::from(format!(
                "{:.2}% {}",
                calculate_percentage(
                    state.synchronization.incoming_transfer.current_block_count,
                    state.synchronization.incoming_transfer.downloaded_blocks
                ),
                convert_eta(syncing_eta)
            )),
            Spans::from(format!(
                "{} level",
                state.synchronization.incoming_transfer.downloaded_blocks
            )),
            Spans::from(format!(
                "{:.2} blocks / s",
                state.synchronization.incoming_transfer.download_rate
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

        let application_eta = if state
            .synchronization
            .aplication_status
            .current_application_speed
            != 0.0
        {
            (state.synchronization.incoming_transfer.current_block_count
                - state.last_applied_level as usize) as f32
                / state
                    .synchronization
                    .aplication_status
                    .current_application_speed
                * 60.0
        } else {
            0.0
        };

        let paragraph = Paragraph::new(vec![
            Spans::from(format!(
                "{:.2}% {}",
                calculate_percentage(
                    state.synchronization.incoming_transfer.current_block_count,
                    state.last_applied_level as usize
                ),
                convert_eta(application_eta)
            )),
            Spans::from(format!("{} level", state.last_applied_level)),
            Spans::from(format!(
                "{:.2} blocks / s",
                state
                    .synchronization
                    .aplication_status
                    .current_application_speed
                    / 60.0
            )),
        ])
        .style(Style::default())
        .block(applying)
        .alignment(Alignment::Left);
        f.render_widget(paragraph, top_chunks[1]);

        // ======================== CHAIN STATUS ========================
        // let chain_status = Block::default().borders(Borders::ALL);
        // f.render_widget(chain_status, page_chunks[1]);

        let cycle_block_width = 5;
        let cycle_block_heigth = 3;
        let period_block_width = 42;
        let period_block_height = cycle_block_heigth + 3;

        let cycle_per_period = 8;
        let period_count_per_page_on_heigth = page_chunks[1].height / period_block_height;
        let period_count_per_page_on_width = page_chunks[1].width / period_block_width;

        let vertical_padding =
            (page_chunks[1].height - (period_count_per_page_on_heigth * period_block_height)) / 2;
        let horizontal_padding =
            (page_chunks[1].width - (period_count_per_page_on_width * period_block_width)) / 2;

        let cycle_count = state.synchronization.cycle_data.len();
        let period_count = cycle_count / cycle_per_period;
        // let period_count = divide_round_up(cycle_count, cycle_per_period);

        // selected period container state
        // ui_state.period_info_state.displayable_container_count =
        //     period_count_per_page_on_heigth.into();
        // ui_state.period_info_state.container_count =
        //     period_count / period_count_per_page_on_width as usize;
        // ui.ui_state.period_info_state.container_count = divide_round_up(period_count, period_count_per_page_on_width as usize);

        // if let Some(selected_container) = ui.ui_state.period_info_state.selected {
        //     if selected_container >= ui.ui_state.period_info_state.displayable_container_count {
        //         ui.ui_state.period_info_state.selected = Some(ui.ui_state.period_info_state.displayable_container_count - 1)
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
            .split(page_chunks[1])[1];

        let periods_containers = Layout::default()
            .direction(Direction::Vertical)
            .constraints(period_containers_row_constraints)
            .split(periods_containers_chunk);

        let applied_style = Style::default().bg(Color::Cyan).fg(Color::Black);
        let dowloaded_style = Style::default().bg(Color::Gray).fg(Color::Black);
        let default_style = Style::default().bg(Color::Black).fg(Color::Black);

        // TODO: more appropriate approach
        // do not render while we have no data
        if state.synchronization.block_metrics.is_empty() {
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

            if let Some(selected_container) = state.synchronization.period_info_state.selected {
                if matches!(widget_in_focus, ActiveWidget::PeriodInfo)
                    && selected_container
                        == container_index + state.synchronization.period_info_state.offset()
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
                if (container_index + state.synchronization.period_info_state.offset())
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
                        + state.synchronization.period_info_state.offset())
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
                        .style(
                            if state.synchronization.block_metrics.len() <= cycle_data_index {
                                default_style
                            } else if state.synchronization.cycle_data[cycle_data_index]
                                .all_applied()
                            {
                                applied_style
                            } else if state.synchronization.block_metrics[cycle_data_index]
                                .all_downloaded()
                            {
                                dowloaded_style
                            } else {
                                default_style
                            },
                        );
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
                .border_style(Style::default().fg(
                    if matches!(widget_in_focus, ActiveWidget::PeerTable) {
                        Color::Blue
                    } else {
                        Color::White
                    },
                ));
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
        let rows = state.synchronization.peer_metrics.iter().map(|item| {
            let item = item.to_table_representation();
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
        if matches!(widget_in_focus, ActiveWidget::PeerTable) {
            f.render_stateful_widget(
                table,
                page_chunks[2],
                &mut state.synchronization.peer_table_state.clone(),
            );
        } else {
            f.render_widget(table, page_chunks[2]);
        }

        // ======================== PAGES TABS ========================
        let tabs = create_pages_tabs(&state.ui);
        f.render_widget(tabs, page_chunks[3]);

        // ======================== HELP BAR ========================
        create_help_bar(page_chunks[4], f, delta_toggle);
    }
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

fn _divide_round_up(dividend: usize, divisor: usize) -> usize {
    (dividend + (divisor - 1)) / divisor
}
