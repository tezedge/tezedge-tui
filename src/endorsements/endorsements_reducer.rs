use crate::automaton::{Action, ActionWithMeta, State};

pub fn endorsementrs_reducer(state: &mut State, action: &ActionWithMeta) {
    let action_time = action.time_as_nanos();

    match &action.action {
        Action::Init(_) => todo!(),
        Action::EndorsementsRightsGet(_) => {}
        Action::RpcRequest(_) => {}
        Action::RpcResponse(action) => {}
        Action::EndorsementsRightsReceived(action) => {
            state.endorsmenents.endorsement_rights = action.endorsement_rights.clone();
        }
        Action::CurrentHeadHeaderReceived(action) => {
            state.current_head_header = action.current_head_header.clone();
        }
        _ => {}
    }
}
