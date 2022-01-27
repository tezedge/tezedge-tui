use crossterm::{
    event::{DisableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use std::time::{Duration, SystemTime};
use tokio::sync::mpsc;
use url::Url;

use slog::Logger;

pub use crate::services::{Service, ServiceDefault};
use crate::{
    endorsements::{
        CurrentHeadHeaderGetAction, EndorsementsRightsGetAction, EndorsementsStatusesGetAction,
    },
    operations::OperationsStatisticsGetAction,
    rpc::RpcResponseReadAction,
    services::{
        rpc_service::RpcServiceDefault,
        tui_service::{TuiService, TuiServiceDefault},
        ws_service::WebsocketServiceDefault,
    },
    terminal_ui::{
        ActivePage, ChangeScreenAction, DrawScreenAction, TuiDeltaToggleKeyPushedAction,
        TuiDownKeyPushedAction, TuiEvent, TuiLeftKeyPushedAction, TuiRightKeyPushedAction,
        TuiSortKeyPushedAction, TuiUpKeyPushedAction, TuiWidgetSelectionKeyPushedAction,
    },
    websocket::WebsocketReadAction,
};

use super::{effects, reducer, Action, ShutdownAction, State};

pub type Store<Service> = redux_rs::Store<State, Service, Action>;

pub struct Automaton<Serv> {
    store: Store<Serv>,
}

impl<Serv: Service> Automaton<Serv> {
    pub fn new(initial_state: State, service: Serv) -> Self {
        let store = Store::new(reducer, effects, service, SystemTime::now(), initial_state);

        Self { store }
    }

    pub async fn make_progress(&mut self, events: &mut mpsc::Receiver<TuiEvent>) {
        loop {
            self.store.dispatch(RpcResponseReadAction {});
            self.store.dispatch(DrawScreenAction {});
            match events.recv().await {
                Some(TuiEvent::Tick) => {
                    self.store.dispatch(WebsocketReadAction {});
                    self.store.dispatch(CurrentHeadHeaderGetAction {});
                    self.store.dispatch(EndorsementsRightsGetAction {
                        block: self.store.state().current_head_header.hash.clone(),
                        level: self.store.state().current_head_header.level,
                    });
                    self.store.dispatch(EndorsementsStatusesGetAction {});
                }
                Some(TuiEvent::Input(key, modifier)) => match key {
                    KeyCode::Char('q') => {
                        self.store.dispatch(ShutdownAction {});
                        return;
                    }
                    KeyCode::Char('s') => {
                        self.store.dispatch(TuiSortKeyPushedAction { modifier });
                    }
                    KeyCode::Char('d') => {
                        self.store.dispatch(TuiDeltaToggleKeyPushedAction {});
                    }
                    KeyCode::F(1) => {
                        self.store.dispatch(ChangeScreenAction {
                            screen: ActivePage::Synchronization,
                        });
                    }
                    KeyCode::F(2) => {
                        self.store.dispatch(ChangeScreenAction {
                            screen: ActivePage::Mempool,
                        });
                    }
                    KeyCode::F(3) => {
                        self.store.dispatch(OperationsStatisticsGetAction {});
                        self.store.dispatch(ChangeScreenAction {
                            screen: ActivePage::Statistics,
                        });
                    }
                    KeyCode::Tab => {
                        self.store.dispatch(TuiWidgetSelectionKeyPushedAction {});
                    }
                    KeyCode::Right => {
                        self.store.dispatch(TuiRightKeyPushedAction {});
                    }
                    KeyCode::Left => {
                        self.store.dispatch(TuiLeftKeyPushedAction {});
                    }
                    KeyCode::Down => {
                        self.store.dispatch(TuiDownKeyPushedAction {});
                    }
                    KeyCode::Up => {
                        self.store.dispatch(TuiUpKeyPushedAction {});
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

impl<Serv> Clone for Automaton<Serv>
where
    Serv: Clone,
{
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}

pub struct AutomatonManager {
    automaton: Automaton<ServiceDefault>,
    tui_event_receiver: mpsc::Receiver<TuiEvent>,
}

impl AutomatonManager {
    const MPCS_QUEUE_MAX_CAPACITY: usize = 4096;

    pub fn new(rpc_url: Url, websocket_url: Url, log: Logger) -> Self {
        let rpc_service = RpcServiceDefault::new(Self::MPCS_QUEUE_MAX_CAPACITY, rpc_url, &log);
        let websocket_service =
            WebsocketServiceDefault::new(Self::MPCS_QUEUE_MAX_CAPACITY, websocket_url, &log);
        let tui_service = TuiServiceDefault::new();
        let tui_event_receiver = TuiServiceDefault::start(Duration::from_secs(1));

        let service = ServiceDefault {
            rpc: rpc_service,
            tui: tui_service,
            ws: websocket_service,
        };

        let initial_state = State::new(log.clone());

        let automaton = Automaton::new(initial_state, service);

        Self {
            automaton,
            tui_event_receiver,
        }
    }

    pub async fn start(&mut self) {
        self.automaton
            .make_progress(&mut self.tui_event_receiver)
            .await;

        // cleanup the terminal on shutdown
        let backend_mut = self
            .automaton
            .store
            .service()
            .tui()
            .terminal()
            .backend_mut();
        execute!(backend_mut, LeaveAlternateScreen, DisableMouseCapture)
            .expect("Error occured while restoring terminal. Please restart your session.");
        disable_raw_mode().expect("Error while dissabling raw mode. Please restart your session");
        self.automaton
            .store
            .service()
            .tui()
            .terminal()
            .show_cursor()
            .expect("Error while restoring cursor. Please restart your session");
    }
}
