use crate::{
    automaton::{Action, ActionWithMeta, Store},
    rpc::RpcRequestAction,
    services::{
        rpc_service::{RpcCall, RpcTarget},
        Service,
    },
};

use super::BakingRightsGetAction;

pub fn baking_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    match &action.action {
        Action::ApplicationStatisticsGet(action) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(
                    RpcTarget::ApplicationStatistics,
                    Some(format!("?level={}", action.level)),
                ),
            });
        }
        Action::PerPeerBlockStatisticsGet(action) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(
                    RpcTarget::PerPeerBlockStatistics,
                    Some(format!("?level={}", action.level)),
                ),
            });
        }
        Action::CycleChanged(action) => {
            if let Some(delegate) = store.state().baker_address.clone() {
                store.dispatch(RpcRequestAction {
                    call: RpcCall::new(
                        RpcTarget::BakingRights,
                        Some(format!(
                            "?delegate={}&max_priority=0&cycle={}",
                            delegate, action.new_cycle
                        )),
                    ),
                });
            }
        }
        Action::BakingRightsGet(_) => {
            // TODO: change this to correnct cycle
            let cycle = store.state().current_head_header.level / 4096;
            if let Some(delegate) = store.state().baker_address.clone() {
                store.dispatch(RpcRequestAction {
                    call: RpcCall::new(
                        RpcTarget::BakingRights,
                        Some(format!(
                            "?delegate={}&max_priority=0&cycle={}",
                            delegate, cycle
                        )),
                    ),
                });
            }
        }
        Action::CurrentHeadHeaderChanged(_) => {
            let is_empty = store.state().baking.baking_rights.rights.is_empty();
            if is_empty {
                store.dispatch(BakingRightsGetAction {});
            }
        }
        _ => {}
    }
}
