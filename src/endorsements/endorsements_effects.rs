use crate::{
    automaton::{Action, ActionWithMeta, Store},
    rpc::RpcRequestAction,
    services::{
        rpc_service::{RpcCall, RpcTarget},
        Service,
    },
};

pub fn endorsement_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    match &action.action {
        Action::Init(_) => {}
        Action::EndorsementsRightsGet(action) => {
            store.dispatch(RpcRequestAction {
                // call: RpcCall::new(RpcTarget::EndorsementRights, Some(String::from("?level=300000&block=BKxQ6om5eQYNdAEokJ4DVYwiASabB9FLJzeUedP34b3Wds79NUJ"))),
                call: RpcCall::new(
                    RpcTarget::EndorsementRights,
                    Some(format!("?level{}&block={}", action.level, action.block)),
                ),
            });
        }
        Action::EndorsementsStatusesGet(_) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(RpcTarget::EndersementsStatus, None),
            });
        }
        Action::CurrentHeadHeaderGet(_) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(RpcTarget::CurrentHeadHeader, None),
            });
        }
        Action::DrawEndorsementsScreen(_) => {}
        _ => {}
    }
}
