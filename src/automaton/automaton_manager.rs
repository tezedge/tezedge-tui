use crossterm::{
    event::{DisableMouseCapture, KeyCode, KeyModifiers},
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
    services::{
        rpc_service::RpcServiceDefault,
        tui_service::{TuiService, TuiServiceDefault},
        ws_service::WebsocketServiceDefault,
    },
    terminal_ui::{
        ActivePage, ChangeScreenAction, DrawScreenAction, TuiEvent, TuiLeftKeyPushedAction,
        TuiRightKeyPushedAction, TuiSortKeyPushedAction, TuiDeltaToggleKeyPushedAction, TuiWidgetSelectionKeyPushedAction, TuiDownKeyPushedAction, TuiUpKeyPushedAction,
    },
    websocket::WebsocketReadAction,
};

use super::{effects, reducer, Action, ShutdownAction, State};

pub type Store<Service> = redux_rs::Store<State, Service, Action>;

enum AutomatonThreadHandle {
    Running(std::thread::JoinHandle<()>),
    NotRunning(Automaton<ServiceDefault>, mpsc::Receiver<TuiEvent>),
}
pub struct Automaton<Serv> {
    /// Container for internal events.
    // events: Events,
    store: Store<Serv>,
}

impl<Serv: Service> Automaton<Serv> {
    pub fn new(initial_state: State, service: Serv /*events: Events*/) -> Self {
        let store = Store::new(reducer, effects, service, SystemTime::now(), initial_state);

        Self { store }
    }

    pub async fn make_progress(&mut self, events: &mut mpsc::Receiver<TuiEvent>) {
        loop {
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
                        self.store.dispatch(TuiSortKeyPushedAction {
                            modifier
                        });
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
            // events: self.events.clone(),
            store: self.store.clone(),
        }
    }
}

pub struct AutomatonManager {
    automaton_thread_handle: Option<AutomatonThreadHandle>,
    log: Logger,
}

impl AutomatonManager {
    const AUTOMATON_QUEUE_MAX_CAPACITY: usize = 100_000;

    pub fn new(rpc_url: Url, websocket_url: Url, log: Logger) -> Self {
        let rpc_service = RpcServiceDefault::new(4096, rpc_url, &log);
        let websocket_service = WebsocketServiceDefault::new(4096, websocket_url, &log);
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
            log,
            automaton_thread_handle: Some(AutomatonThreadHandle::NotRunning(
                automaton,
                tui_event_receiver,
            )),
        }
    }

    pub async fn start(&mut self) {
        if let Some(AutomatonThreadHandle::NotRunning(mut automaton, mut tui_event_receiver)) =
            self.automaton_thread_handle.take()
        {
            automaton.make_progress(&mut tui_event_receiver).await;

            // cleanup the terminal on shutdown
            let backend_mut = automaton.store.service().tui().terminal().backend_mut();
            execute!(backend_mut, LeaveAlternateScreen, DisableMouseCapture);
            disable_raw_mode();
            automaton.store.service().tui().terminal().show_cursor();
        }
    }
}
