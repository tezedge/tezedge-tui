use redux_rs::chain_reducers;

use crate::{
    baking::baking_reducer, endorsements::endorsementrs_reducer, operations::operations_reducer,
    synchronization::synchronization_reducer, terminal_ui::tui_reducer,
};

use super::{ActionWithMeta, State};

pub fn reducer(state: &mut State, action: &ActionWithMeta) {
    chain_reducers!(
        state,
        action,
        tui_reducer,
        synchronization_reducer,
        endorsementrs_reducer,
        baking_reducer,
        operations_reducer
    );
}
