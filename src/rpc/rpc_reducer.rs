use crate::automaton::{Action, ActionWithMeta, State};

use super::RpcState;

pub fn rpc_reducer(state: &mut State, action: &ActionWithMeta) {
    let action_time = action.time_as_nanos();

    match &action.action {
        Action::RpcRequest(_) => match state.rpc_state {
            RpcState::Idle | RpcState::Success => state.rpc_state = RpcState::Pending,
            RpcState::Pending => {}
        },
        Action::RpcResponse(_) => match state.rpc_state {
            RpcState::Idle | RpcState::Success => {}
            RpcState::Pending => state.rpc_state = RpcState::Success,
        },
        _ => {}
    }
}
