use std::time::{Duration, SystemTime};
use url::Url;

use slog::Logger;

pub use crate::services::{Service, ServiceDefault};
use crate::{endorsements::EndorsementsRightsGetAction, services::rpc_service::RpcServiceDefault};

use super::{effects, reducer, Action, State};

pub type Store<Service> = redux_rs::Store<State, Service, Action>;

enum AutomatonThreadHandle {
    Running(std::thread::JoinHandle<()>),
    NotRunning(Automaton<ServiceDefault>),
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

    pub fn make_progress(&mut self) {
        // TODO
        println!("Progress");
        self.store.dispatch(EndorsementsRightsGetAction {
            block: String::from("BKxQ6om5eQYNdAEokJ4DVYwiASabB9FLJzeUedP34b3Wds79NUJ"),
            level: 300000,
        });
        std::thread::sleep(Duration::from_secs(1));
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
        let rpc_service = RpcServiceDefault::new(4096, url);

        let service = ServiceDefault { rpc: rpc_service };

        let initial_state = State::default();

        let automaton = Automaton::new(initial_state, service);

        Self {
            log,
            automaton_thread_handle: Some(AutomatonThreadHandle::NotRunning(automaton)),
        }
    }

    pub fn start(&mut self) {
        if let Some(AutomatonThreadHandle::NotRunning(mut automaton)) =
            self.automaton_thread_handle.take()
        {
            let automaton_thread_handle = std::thread::Builder::new()
                .name("tui-automaton".to_owned())
                .spawn(move || loop {
                    automaton.make_progress();
                });
        }
    }
}
