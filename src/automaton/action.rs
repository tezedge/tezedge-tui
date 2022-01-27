use derive_more::From;
use enum_kinds::EnumKind;
pub use redux_rs::{ActionId, EnablingCondition};

use crate::{
    endorsements::{
        CurrentHeadHeaderGetAction, CurrentHeadHeaderRecievedAction, EndorsementsRightsGetAction,
        EndorsementsRightsReceivedAction, EndorsementsStatusesGetAction,
        EndorsementsStatusesReceivedAction,
    },
    operations::{OperationsStatisticsGetAction, OperationsStatisticsReceivedAction},
    rpc::{RpcRequestAction, RpcResponseAction, RpcResponseReadAction},
    terminal_ui::{
        ChangeScreenAction, DrawScreenAction, DrawScreenFailiureAction, DrawScreenSuccessAction,
        TuiDeltaToggleKeyPushedAction, TuiDownKeyPushedAction, TuiLeftKeyPushedAction,
        TuiRightKeyPushedAction, TuiSortKeyPushedAction, TuiUpKeyPushedAction,
        TuiWidgetSelectionKeyPushedAction,
    },
    websocket::{WebsocketMessageReceivedAction, WebsocketReadAction},
};

use super::State;

pub type ActionWithMeta = redux_rs::ActionWithMeta<Action>;

#[derive(Debug, Clone)]
pub struct InitAction {}

impl EnablingCondition<State> for InitAction {
    fn is_enabled(&self, _: &State) -> bool {
        false
    }
}
#[derive(Debug, Clone)]
pub struct ShutdownAction {}

impl EnablingCondition<State> for ShutdownAction {
    fn is_enabled(&self, _: &State) -> bool {
        false
    }
}

#[derive(EnumKind, strum_macros::AsRefStr, strum_macros::IntoStaticStr, From, Debug, Clone)]
#[enum_kind(
    ActionKind,
    derive(strum_macros::EnumIter, strum_macros::Display, Hash)
)]
pub enum Action {
    Init(InitAction),
    Shutdown(ShutdownAction),

    RpcRequest(RpcRequestAction),
    RpcResponse(RpcResponseAction),
    RpcResponseRead(RpcResponseReadAction),

    WebsocketRead(WebsocketReadAction),
    WebsocketMessageReceived(WebsocketMessageReceivedAction),

    EndorsementsRightsGet(EndorsementsRightsGetAction),
    EndorsementsRightsReceived(EndorsementsRightsReceivedAction),
    EndorsementsStatusesGet(EndorsementsStatusesGetAction),
    EndorsementsStatusesReceived(EndorsementsStatusesReceivedAction),
    CurrentHeadHeaderGet(CurrentHeadHeaderGetAction),
    CurrentHeadHeaderReceived(CurrentHeadHeaderRecievedAction),

    OperationsStatisticsGet(OperationsStatisticsGetAction),
    OperationsStatisticsReceived(OperationsStatisticsReceivedAction),

    ChangeScreen(ChangeScreenAction),
    DrawScreen(DrawScreenAction),
    DrawScreenSuccess(DrawScreenSuccessAction),
    DrawScreenFailiure(DrawScreenFailiureAction),
    TuiRightKeyPushed(TuiRightKeyPushedAction),
    TuiLeftKeyPushed(TuiLeftKeyPushedAction),
    TuiUpKeyPushedAction(TuiUpKeyPushedAction),
    TuiDownKeyPushedAction(TuiDownKeyPushedAction),
    TuiSortKeyPushed(TuiSortKeyPushedAction),
    TuiDeltaToggleKeyPushed(TuiDeltaToggleKeyPushedAction),
    TuiWidgetSelectionKeyPushed(TuiWidgetSelectionKeyPushedAction),
}

impl Action {
    #[inline(always)]
    pub fn kind(&self) -> ActionKind {
        ActionKind::from(self)
    }
}

impl<'a> From<&'a ActionWithMeta> for ActionKind {
    fn from(action: &'a ActionWithMeta) -> ActionKind {
        action.action.kind()
    }
}

impl From<ActionWithMeta> for ActionKind {
    fn from(action: ActionWithMeta) -> ActionKind {
        action.action.kind()
    }
}
