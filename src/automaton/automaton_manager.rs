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
    endorsements::{CurrentHeadHeaderGetAction, EndorsementsRightsGetAction},
    services::{
        rpc_service::RpcServiceDefault,
        tui_service::{TuiService, TuiServiceDefault},
    },
    terminal_ui::{DrawScreenAction, TuiEvent},
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
        // let events = self.store.service().tui().receiver();
        loop {
            self.store.dispatch(DrawScreenAction {});
            match events.recv().await {
                Some(TuiEvent::Tick) => {
                    self.store.dispatch(CurrentHeadHeaderGetAction {});
                    self.store.dispatch(EndorsementsRightsGetAction {
                        block: self.store.state().current_head_header.hash.clone(),
                        level: self.store.state().current_head_header.level,
                    });
                }
                Some(TuiEvent::Input(key, modifier)) => match key {
                    KeyCode::Char('q') => {
                        self.store.dispatch(ShutdownAction {});
                        return;
                    }
                    KeyCode::Down => {}
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

    pub fn new(url: Url, log: Logger) -> Self {
        let rpc_service = RpcServiceDefault::new(4096, url, &log);
        let tui_service = TuiServiceDefault::new();
        let tui_event_receiver = TuiServiceDefault::start(Duration::from_secs(1));

        let service = ServiceDefault {
            rpc: rpc_service,
            tui: tui_service,
        };

        let initial_state = State::default();

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
