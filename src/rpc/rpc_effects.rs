use crate::{
    automaton::{Action, ActionWithMeta, Store},
    services::{
        rpc_service_async::RpcService,
        Service,
    },
};

pub fn rpc_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    #[allow(clippy::single_match)]
    match &action.action {
        Action::RpcRequest(action) => {
            let _ = store.service().rpc().request_send(action.call.clone());
        }
        _ => {}
    }
}
