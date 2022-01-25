use std::time::Duration;

use crate::{
    automaton::{Action, ActionWithMeta, Store},
    endorsements::{
        CurrentHeadHeaderRecievedAction, EndorsementsRightsReceivedAction,
        EndorsementsStatusesReceivedAction,
    },
    services::{
        rpc_service::{RpcResponse, RpcService},
        Service,
    },
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

            // TODO: move this to different action + create wakeup event
            while let Ok(response) = store.service().rpc().response_try_recv() {
                store.dispatch(RpcResponseAction { response });
            }
        }
        Action::RpcResponse(action) => match &action.response {
            RpcResponse::EndorsementRights(rights) => {
                let _ = store.dispatch(EndorsementsRightsReceivedAction {
                    endorsement_rights: rights.clone(),
                });
            }
            RpcResponse::EndorsementsStatus(endorsements_statuses) => {
                let _ = store.dispatch(EndorsementsStatusesReceivedAction {
                    endorsements_statuses: endorsements_statuses.clone(),
                });
            }
            RpcResponse::CurrentHeadHeader(current_head_header) => {
                let _ = store.dispatch(CurrentHeadHeaderRecievedAction {
                    current_head_header: current_head_header.clone(),
                });
            }
            RpcResponse::OperationsStats(_) => todo!(),
        },
        _ => {}
    }
}
