use crate::{
    automaton::{Action, ActionWithMeta, Store},
    rpc::RpcRequestAction,
    services::{
        rpc_service::{RpcCall, RpcTarget},
        Service,
    },
};

pub fn operations_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    match &action.action {
        Action::Init(_) => {}
        Action::OperationsStatisticsGet(_) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(RpcTarget::OperationsStats, None),
            });
        }
        _ => {}
    }
}
