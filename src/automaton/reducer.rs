use redux_rs::chain_reducers;

use crate::{endorsements::endorsementrs_reducer, rpc::rpc_reducer};

use super::{ActionWithMeta, State};

pub fn reducer(state: &mut State, action: &ActionWithMeta) {
    chain_reducers!(state, action, rpc_reducer, endorsementrs_reducer);
}
