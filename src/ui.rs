use std::io;

use crossterm::event::{self, Event, KeyCode};
use tokio::sync::mpsc;
use tokio::time::Duration;
use tui::{
    backend::Backend,
    Terminal,
};

use crate::layout::{MempoolScreen, SyncingScreen};
use crate::node_rpc::Node;

use crate::model::{RollingList, StateRef, UiState};
#[derive(Default)]
pub struct Ui {
    pub state: StateRef,
    pub ui_state: UiState,
    pub node: Node,
}

impl Ui {
    // TODO: add constructor function, rework the url..
    pub async fn run_tui<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> io::Result<()> {
        let mut events = events(tick_rate);
        loop {
            let page_in_focus = self.ui_state.page_state.in_focus();

            let data_state = &self.state;
            let ui_state = &mut self.ui_state;

            // Note: here we decide what screen to draw
            terminal.draw(|f| match page_in_focus {
                0 => SyncingScreen::draw_syncing_screen::<B>(data_state, ui_state, f),
                1 => MempoolScreen::draw_mempool_screen::<B>(data_state, ui_state, f),
                _ => {}
            })?;

            match events.recv().await {
                Some(TuiEvent::Input(key)) => match key {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => self.next(),
                    KeyCode::Up => self.previous(),
                    KeyCode::Right => self.ui_state.page_state.next(),
                    KeyCode::Left => self.ui_state.page_state.previous(),
                    KeyCode::Tab => {
                        self.ui_state.page_state.pages[page_in_focus].widgets.next();
                    }
                    _ => {}
                },
                Some(TuiEvent::Tick) => {
                    let mut state = self.state.write().unwrap();
                    state.update_current_head_header(&self.node).await;
                    state.update_endorsers(&self.node).await;
                }
                None => return Ok(()),
                _ => {}
            }
        }
    }

    pub fn next(&mut self) {
        let page_in_focus = self.ui_state.page_state.in_focus();
        match page_in_focus {
            // syncing page
            0 => {
                let widget_in_focus = self.ui_state.page_state.pages[page_in_focus]
                    .widgets
                    .in_focus();
                match widget_in_focus {
                    // peer table widget
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
                    // period blocks
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
            // mempool page
            1 => {
                // control widgets on mempool page
            }
            _ => {}
        }
    }

    pub fn previous(&mut self) {
        let page_in_focus = self.ui_state.page_state.in_focus();
        match page_in_focus {
            // syncing page
            0 => {
                let widget_in_focus = self.ui_state.page_state.pages[page_in_focus]
                    .widgets
                    .in_focus();
                match widget_in_focus {
                    // peer table widget
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
                    // period blocks
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
            // mempool page
            1 => {
                // control widgets on mempool page
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
