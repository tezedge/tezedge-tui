use crossterm::event::KeyCode;
use std::time::{Duration, SystemTime};
use tokio::sync::mpsc;
use url::Url;

use slog::Logger;

pub use crate::services::{Service, ServiceDefault};
use crate::{
    baking::{
        ApplicationStatisticsGetAction, ApplicationStatisticsReceivedAction,
        BakingRightsReceivedAction, PerPeerBlockStatisticsGetAction,
        PerPeerBlockStatisticsReceivedAction,
    },
    endorsements::{
        EndorsementsRightsReceivedAction, EndorsementsRightsWithTimeReceivedAction,
        EndorsementsStatusesGetAction, EndorsementsStatusesReceivedAction,
        MempoolEndorsementStatsGetAction, MempoolEndorsementStatsReceivedAction,
    },
    operations::{OperationsStatisticsGetAction, OperationsStatisticsReceivedAction},
    services::{
        rpc_service_async::{RpcResponse, RpcService, RpcServiceDefault},
        tui_service::{TuiService, TuiServiceDefault},
        ws_service::WebsocketServiceDefault,
    },
    terminal_ui::{
        ActivePage, BestRemoteLevelGetAction, BestRemoteLevelReceivedAction, ChangeScreenAction,
        CurrentHeadHeaderGetAction, CurrentHeadHeaderRecievedAction, CurrentHeadMetadataGetAction,
        CurrentHeadMetadataReceivedAction, DrawScreenAction, NetworkConstantsGetAction,
        NetworkConstantsReceivedAction, TuiDeltaToggleKeyPushedAction, TuiDownKeyPushedAction,
        TuiEvent, TuiLeftKeyPushedAction, TuiRightKeyPushedAction, TuiSortKeyPushedAction,
        TuiUpKeyPushedAction, TuiWidgetSelectionKeyPushedAction,
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
        self.store.dispatch(NetworkConstantsGetAction {});
        loop {
            // TODO: clean this up (create handler functions)
            self.store.dispatch(DrawScreenAction {});
            tokio::select! {
                tui_event = events.recv() => {
                    match tui_event {
                        Some(TuiEvent::Tick) => {
                            self.store.dispatch(WebsocketReadAction {});
                            self.store.dispatch(BestRemoteLevelGetAction {});
                            self.store.dispatch(CurrentHeadHeaderGetAction {});
                            self.store.dispatch(CurrentHeadMetadataGetAction {});

                            self.store.dispatch(EndorsementsStatusesGetAction {});
                            self.store.dispatch(ApplicationStatisticsGetAction {
                                level: self.store.state().current_head_header.level,
                            });
                            self.store.dispatch(PerPeerBlockStatisticsGetAction {
                                level: self.store.state().current_head_header.level,
                            });
                            self.store.dispatch(MempoolEndorsementStatsGetAction {});
                        }
                        Some(TuiEvent::Input(key, modifier)) => match key {
                            KeyCode::F(10) => {
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
                                    screen: ActivePage::Endorsements,
                                });
                            }
                            KeyCode::F(3) => {
                                self.store.dispatch(OperationsStatisticsGetAction {});
                                self.store.dispatch(ChangeScreenAction {
                                    screen: ActivePage::Statistics,
                                });
                            }
                            KeyCode::F(4) => {
                                self.store.dispatch(ChangeScreenAction {
                                    screen: ActivePage::Baking,
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
                rpc_response = self.store.service().rpc().response_recv() => {
                    if let Some(resp) = rpc_response {
                        match resp {
                            RpcResponse::EndorsementRights(rights) => {
                                self.store.dispatch(EndorsementsRightsReceivedAction {
                                    endorsement_rights: rights.clone(),
                                });
                            }
                            RpcResponse::EndorsementsStatus(endorsements_statuses) => {
                                self.store.dispatch(EndorsementsStatusesReceivedAction {
                                    endorsements_statuses: endorsements_statuses.clone(),
                                });
                            }
                            RpcResponse::CurrentHeadHeader(current_head_header) => {
                                self.store.dispatch(CurrentHeadHeaderRecievedAction {
                                    current_head_header: current_head_header.clone(),
                                });
                            }
                            RpcResponse::OperationsStats(operations_statistics) => {
                                self.store.dispatch(OperationsStatisticsReceivedAction {
                                    operations_statistics: operations_statistics.clone(),
                                });
                            }
                            RpcResponse::ApplicationStatistics(application_stats) => {
                                self.store.dispatch(ApplicationStatisticsReceivedAction {
                                    application_statistics: application_stats.clone(),
                                });
                            }
                            RpcResponse::PerPeerBlockStatistics(per_peer_stats) => {
                                self.store.dispatch(PerPeerBlockStatisticsReceivedAction {
                                    per_peer_block_statistics: per_peer_stats.clone(),
                                });
                            }
                            RpcResponse::BakingRights(rights) => {
                                self.store.dispatch(BakingRightsReceivedAction {
                                    rights: rights.clone(),
                                });
                            }
                            RpcResponse::EndorsementRightsWithTime(rights) => {
                                self.store.dispatch(EndorsementsRightsWithTimeReceivedAction {
                                    rights: rights.clone(),
                                });
                            }
                            RpcResponse::MempoolEndorsementStats(stats) => {
                                self.store.dispatch(MempoolEndorsementStatsReceivedAction {
                                    stats: stats.clone(),
                                });
                            }
                            RpcResponse::NetworkConstants(constants) => {
                                self.store.dispatch(NetworkConstantsReceivedAction {
                                    constants: constants.clone(),
                                });
                            }
                            RpcResponse::CurrentHeadMetadata(meta) => {
                                self.store.dispatch(CurrentHeadMetadataReceivedAction {
                                    metadata: meta.clone()
                                });
                            }
                            RpcResponse::BestRemoteLevel(level) => {
                                self.store.dispatch(BestRemoteLevelReceivedAction {
                                    level
                                });
                            }
                        }
                    }
                }
            }
            // match events.recv().await {

            // }
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

    pub fn new(
        rpc_url: Url,
        websocket_url: Url,
        baker_address: Option<String>,
        log: Logger,
    ) -> Self {
        let rpc_service = RpcServiceDefault::new(Self::MPCS_QUEUE_MAX_CAPACITY, rpc_url, &log);
        let websocket_service =
            WebsocketServiceDefault::new(Self::MPCS_QUEUE_MAX_CAPACITY, websocket_url, &log);
        let tui_service = TuiServiceDefault::new();
        let tui_event_receiver = TuiServiceDefault::start(Duration::from_millis(1000));

        let service = ServiceDefault {
            rpc: rpc_service,
            tui: tui_service,
            ws: websocket_service,
        };

        let initial_state = State::new(baker_address, log.clone());

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

        self.automaton.store.service().tui().restore_terminal();
    }
}
