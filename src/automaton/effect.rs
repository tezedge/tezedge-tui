use crate::{
    endorsements::endorsements_effects::endorsement_effects, rpc::rpc_effects::rpc_effects,
};

use super::{action, ActionWithMeta, Service, Store};

pub fn effects<S: Service>(store: &mut Store<S>, action: &ActionWithMeta) {
    rpc_effects(store, action);
    endorsement_effects(store, action);
}
