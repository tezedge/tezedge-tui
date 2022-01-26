use crate::{
    endorsements::endorsements_effects::endorsement_effects, operations::operations_effects,
    rpc::rpc_effects::rpc_effects, terminal_ui::tui_effects, websocket::websocket_effects,
};

use super::{ActionWithMeta, Service, Store};

pub fn effects<S: Service>(store: &mut Store<S>, action: &ActionWithMeta) {
    tui_effects(store, action);
    websocket_effects(store, action);
    rpc_effects(store, action);
    endorsement_effects(store, action);
    operations_effects(store, action);
}
