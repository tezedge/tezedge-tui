use redux_rs::EnablingCondition;
use serde::{Serialize, Deserialize};

use crate::{
    automaton::State,
    services::rpc_service_async::{RpcCall, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequestAction {
    pub call: RpcCall,
}

impl EnablingCondition<State> for RpcRequestAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponseAction {
    pub response: RpcResponse,
}

impl EnablingCondition<State> for RpcResponseAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponseReadAction {}

impl EnablingCondition<State> for RpcResponseReadAction {
    fn is_enabled(&self, _: &State) -> bool {
        true
    }
}
