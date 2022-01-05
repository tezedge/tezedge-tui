use std::io;

use crossterm::event::{self, Event, KeyCode};
use slog::{info, Logger};
use tokio::sync::mpsc;
use tokio::time::Duration;
use tui::{backend::Backend, Terminal};

use crate::configuration::TuiArgs;
use crate::layout::{MempoolScreen, StatisticsScreen, SyncingScreen};
use crate::node_rpc::Node;

use crate::model::{ActivePage, ActiveWidget, SortableByFocus, StateRef, UiState};
pub struct Ui {
    pub state: StateRef,
    pub ui_state: UiState,
    pub node: Node,
    pub log: Logger,
}

impl Ui {
    pub fn new(args: &TuiArgs) -> Self {
        let logger = create_file_logger("tui.log");
        Self {
            state: Default::default(),
            ui_state: Default::default(),
            node: Node::new(&args.node, logger.clone()),
            log: logger,
        }
    }

    // TODO: Error handling (unwraps)
    pub async fn run_tui<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut events = events(tick_rate);
        loop {
            let data_state = &self.state;
            let ui_state = &mut self.ui_state;
            let active_page = ui_state.active_page.clone();
            let log = self.log.clone();
            // Note: here we decide what screen to draw
            terminal.draw(|f| match active_page {
                ActivePage::Synchronization => {
                    SyncingScreen::draw_syncing_screen::<B>(data_state, ui_state, f)
                }
                ActivePage::Mempool => {
                    MempoolScreen::draw_mempool_screen::<B>(data_state, ui_state, f)
                }
                ActivePage::Statistics => {
                    StatisticsScreen::draw_statistics_screen::<B>(data_state, ui_state, &log, f)
                }
            })?;

            match events.recv().await {
                Some(TuiEvent::Input(key)) => match key {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => self.handle_down(),
                    KeyCode::Up => self.handle_up(),
                    KeyCode::Right => {}
                    KeyCode::Left => {}
                    KeyCode::Tab => {
                        self.rotate_widgets();
                    }
                    KeyCode::Char('k') => {
                        self.ui_state.endorsement_sorter_state.next();
                        self.state
                            .write()
                            .map(|mut state| {
                                state.current_head_endorsement_statuses.sort_by_focus(
                                    self.ui_state.endorsement_sorter_state.in_focus(),
                                )
                            })
                            .unwrap();
                    }
                    KeyCode::Char('j') => {
                        self.ui_state.endorsement_sorter_state.previous();
                        self.state
                            .write()
                            .map(|mut state| {
                                state.current_head_endorsement_statuses.sort_by_focus(
                                    self.ui_state.endorsement_sorter_state.in_focus(),
                                )
                            })
                            .unwrap();
                    }
                    KeyCode::F(1) => self.ui_state.active_page = ActivePage::Synchronization,
                    KeyCode::F(2) => self.ui_state.active_page = ActivePage::Mempool,
                    KeyCode::F(3) => {
                        self.ui_state.active_page = ActivePage::Statistics;
                        let mut state_write = self.state.write().unwrap();

                        // This call can be very long so we launch a thread, when the flag is not set (a thread is already running)
                        if !state_write.statistics_pending {
                            info!(self.log, "Getting operations statistics");
                            state_write.statistics_pending = true;
                            drop(state_write);

                            let state = self.state.clone();
                            let node = self.node.clone();
                            let log = self.log.clone();
                            tokio::task::spawn(async move {
                                let stats = crate::model::State::update_statistics(&node).await;
                                let mut state = state.write().unwrap();

                                state.operations_statistics = stats;
                                state.statistics_pending = false;
                                info!(log, "Statistics received");
                            });
                        }
                    }
                    _ => {}
                },
                Some(TuiEvent::Tick) => {
                    info!(
                        self.log,
                        "Active Page: {:?}; Active widget: {:?}",
                        ui_state.active_page,
                        ui_state.active_widget
                    );
                    let mut state = self.state.write().unwrap();
                    state
                        .update_current_head_header(
                            &self.node,
                            self.ui_state.endorsement_sorter_state.in_focus(),
                        )
                        .await;
                    state
                        .update_endorsers(
                            &self.node,
                            self.ui_state.endorsement_sorter_state.in_focus(),
                        )
                        .await;
                }
                None => return Ok(()),
                _ => {}
            }
        }
    }

    pub fn rotate_widgets(&mut self) {
        match self.ui_state.active_page {
            ActivePage::Synchronization => match self.ui_state.active_widget {
                ActiveWidget::PeriodInfo => self.ui_state.active_widget = ActiveWidget::PeerTable,
                ActiveWidget::PeerTable => self.ui_state.active_widget = ActiveWidget::PeriodInfo,
                ActiveWidget::EndorserTable => {
                    self.ui_state.active_widget = ActiveWidget::PeriodInfo
                }
            },
            ActivePage::Mempool => match self.ui_state.active_widget {
                ActiveWidget::PeriodInfo => {
                    self.ui_state.active_widget = ActiveWidget::EndorserTable
                }
                ActiveWidget::PeerTable => {
                    self.ui_state.active_widget = ActiveWidget::EndorserTable
                }
                ActiveWidget::EndorserTable => {
                    self.ui_state.active_widget = ActiveWidget::EndorserTable
                }
            },
            ActivePage::Statistics => {
                // TODO
            }
        }
    }

    pub fn handle_up(&mut self) {
        let state = self.state.read().unwrap();
        match self.ui_state.active_widget {
            ActiveWidget::PeriodInfo => self.ui_state.period_info_state.select(previous_item(
                self.ui_state.period_info_state.container_count,
                self.ui_state.period_info_state.selected(),
            )),
            ActiveWidget::PeerTable => self.ui_state.peer_table_state.select(previous_item(
                state.peer_metrics.len(),
                self.ui_state.peer_table_state.selected(),
            )),
            ActiveWidget::EndorserTable => {
                self.ui_state.endorsement_table_state.select(previous_item(
                    state.current_head_endorsement_statuses.len(),
                    self.ui_state.endorsement_table_state.selected(),
                ))
            }
        }
    }

    pub fn handle_down(&mut self) {
        let state = self.state.read().unwrap();
        match self.ui_state.active_widget {
            ActiveWidget::PeriodInfo => self.ui_state.period_info_state.select(next_item(
                self.ui_state.period_info_state.container_count,
                self.ui_state.period_info_state.selected(),
            )),
            ActiveWidget::PeerTable => self.ui_state.peer_table_state.select(next_item(
                state.peer_metrics.len(),
                self.ui_state.peer_table_state.selected(),
            )),
            ActiveWidget::EndorserTable => self.ui_state.endorsement_table_state.select(next_item(
                state.current_head_endorsement_statuses.len(),
                self.ui_state.endorsement_table_state.selected(),
            )),
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

pub fn next_item(total: usize, selection_index: Option<usize>) -> Option<usize> {
    match selection_index {
        Some(selection_index) => {
            if total != 0 {
                let next_index = selection_index + 1;
                if next_index > total - 1 {
                    return Some(0);
                } else {
                    return Some(next_index);
                }
            }
            Some(0)
        }
        None => Some(0),
    }
}

pub fn previous_item(total: usize, selection_index: Option<usize>) -> Option<usize> {
    match selection_index {
        Some(selection_index) => {
            if total != 0 {
                if selection_index > 0 {
                    return Some(selection_index - 1);
                } else {
                    return Some(total - 1);
                }
            }
            Some(0)
        }
        None => Some(0),
    }
}

fn create_file_logger(path: &str) -> slog::Logger {
    use slog::Drain;

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to create log file");

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, slog::o!())
}
