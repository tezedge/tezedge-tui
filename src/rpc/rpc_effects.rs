use crate::{
    automaton::{Action, ActionWithMeta, Store},
    baking::{
        ApplicationStatisticsReceivedAction, BakingRightsReceivedAction,
        PerPeerBlockStatisticsReceivedAction,
    },
    endorsements::{
        EndorsementsRightsReceivedAction, EndorsementsRightsWithTimeReceivedAction,
        EndorsementsStatusesReceivedAction,
    },
    operations::OperationsStatisticsReceivedAction,
    services::{
        rpc_service::{RpcResponse, RpcService},
        Service,
    },
    terminal_ui::CurrentHeadHeaderRecievedAction,
};

use super::RpcResponseAction;

pub fn rpc_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    match &action.action {
        Action::RpcRequest(action) => {
            // TODO: error action/state
            let _ = store.service().rpc().request_send(action.call.clone());
        }
        Action::RpcResponse(action) => match &action.response {
            RpcResponse::EndorsementRights(rights) => {
                store.dispatch(EndorsementsRightsReceivedAction {
                    endorsement_rights: rights.clone(),
                });
            }
            RpcResponse::EndorsementsStatus(endorsements_statuses) => {
                store.dispatch(EndorsementsStatusesReceivedAction {
                    endorsements_statuses: endorsements_statuses.clone(),
                });
            }
            RpcResponse::CurrentHeadHeader(current_head_header) => {
                store.dispatch(CurrentHeadHeaderRecievedAction {
                    current_head_header: current_head_header.clone(),
                });
            }
            RpcResponse::OperationsStats(operations_statistics) => {
                store.dispatch(OperationsStatisticsReceivedAction {
                    operations_statistics: operations_statistics.clone(),
                });
            }
            RpcResponse::ApplicationStatistics(application_stats) => {
                store.dispatch(ApplicationStatisticsReceivedAction {
                    application_statistics: application_stats.clone(),
                });
            }
            RpcResponse::PerPeerBlockStatistics(per_peer_stats) => {
                store.dispatch(PerPeerBlockStatisticsReceivedAction {
                    per_peer_block_statistics: per_peer_stats.clone(),
                });
            }
            RpcResponse::BakingRights(rights) => {
                store.dispatch(BakingRightsReceivedAction {
                    rights: rights.clone(),
                });
            }
            RpcResponse::EndorsementRightsWithTime(rights) => {
                store.dispatch(EndorsementsRightsWithTimeReceivedAction {
                    rights: rights.clone(),
                });
            }
        },
        Action::RpcResponseRead(_) => {
            while let Ok(response) = store.service().rpc().response_try_recv() {
                store.dispatch(RpcResponseAction { response });
            }
        }
        _ => {}
    }
}
