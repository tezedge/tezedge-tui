use crate::{
    automaton::{Action, ActionWithMeta, Store},
    rpc::RpcRequestAction,
    services::{
        rpc_service::{RpcCall, RpcTarget},
        Service,
    },
};

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
        _ => {}
    }
}
