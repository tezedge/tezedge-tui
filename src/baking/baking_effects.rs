use crate::{
    automaton::{Action, ActionWithMeta, Store},
    rpc::RpcRequestAction,
    services::{
        rpc_service_async::{RpcCall, RpcTarget},
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
            if store.state().baker_address.is_some() {
                let preserved_cycles = store.state().network_constants.preserved_cycles;
                store.dispatch(BakingRightsGetAction {
                    cycle: action.new_cycle + preserved_cycles,
                });
            }
        }
        Action::BakingRightsGet(action) => {
            if let Some(delegate) = store.state().baker_address.clone() {
                store.dispatch(RpcRequestAction {
                    call: RpcCall::new(
                        RpcTarget::BakingRights,
                        Some(format!(
                            "?delegate={}&max_priority=0&cycle={}",
                            delegate, action.cycle
                        )),
                    ),
                });
            }
        }
        Action::CurrentHeadMetadataChanged(action) => {
            let is_empty = store.state().baking.baking_rights.rights.is_empty();
            if is_empty {
                let preserved_cycles = store.state().network_constants.preserved_cycles;
                let current_cycle = action.new_metadata.level_info.cycle;

                for cycle in current_cycle..=current_cycle + preserved_cycles {
                    store.dispatch(BakingRightsGetAction { cycle });
                }
            }
        }
        _ => {}
    }
}
