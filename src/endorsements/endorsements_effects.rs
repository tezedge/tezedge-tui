use crate::{
    automaton::{Action, ActionWithMeta, Store},
    rpc::RpcRequestAction,
    services::{
        rpc_service::{RpcCall, RpcTarget},
        Service,
    },
};

use super::EndorsementsRightsWithTimeGetAction;

pub fn endorsement_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    match &action.action {
        Action::EndorsementsStatusesGet(_) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(RpcTarget::EndersementsStatus, None),
            });
        }
        Action::CurrentHeadHeaderChanged(action) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(
                    RpcTarget::EndorsementRights,
                    Some(format!(
                        "?level={}&block={}",
                        action.current_head_header.level, action.current_head_header.hash
                    )),
                ),
            });

            let is_empty = store.state().endorsmenents.endorsement_rights_with_time.rights.is_empty();
            if is_empty {
                store.dispatch(EndorsementsRightsWithTimeGetAction {});
            }
        }
        Action::EndorsementsRightsWithTimeGet(_) => {
            // TODO: change this to correnct cycle
            let cycle = store.state().current_head_header.level / 4096;
            if let Some(delegate) = store.state().baker_address.clone() {
                store.dispatch(RpcRequestAction {
                    call: RpcCall::new(
                        RpcTarget::EndorsementRightsWithTime,
                        Some(format!(
                            "?delegate={}&cycle={}",
                            delegate, cycle
                        )),
                    ),
                });
            }
        }
        _ => {}
    }
}
