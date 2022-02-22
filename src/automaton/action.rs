use derive_more::From;
use enum_kinds::EnumKind;
pub use redux_rs::{ActionId, EnablingCondition};

use crate::{
    baking::{
        ApplicationStatisticsGetAction, ApplicationStatisticsReceivedAction, BakingRightsGetAction,
        BakingRightsReceivedAction, PerPeerBlockStatisticsGetAction,
        PerPeerBlockStatisticsReceivedAction,
    },
    endorsements::{
        EndorsementsRightsGetAction, EndorsementsRightsReceivedAction,
        EndorsementsRightsWithTimeGetAction, EndorsementsRightsWithTimeReceivedAction,
        EndorsementsStatusesGetAction, EndorsementsStatusesReceivedAction,
        MempoolEndorsementStatsGetAction, MempoolEndorsementStatsReceivedAction,
    },
    operations::{OperationsStatisticsGetAction, OperationsStatisticsReceivedAction},
    rpc::{RpcRequestAction, RpcResponseAction, RpcResponseReadAction},
    terminal_ui::{
        BestRemoteLevelChangedAction, BestRemoteLevelGetAction, BestRemoteLevelReceivedAction,
        ChangeScreenAction, CurrentHeadHeaderChangedAction, CurrentHeadHeaderGetAction,
        CurrentHeadHeaderRecievedAction, CurrentHeadMetadataChangedAction,
        CurrentHeadMetadataGetAction, CurrentHeadMetadataReceivedAction, CycleChangedAction,
        DrawScreenAction, DrawScreenFailiureAction, DrawScreenSuccessAction,
        NetworkConstantsGetAction, NetworkConstantsReceivedAction, TuiDeltaToggleKeyPushedAction,
        TuiDownKeyPushedAction, TuiLeftKeyPushedAction, TuiRightKeyPushedAction,
        TuiSortKeyPushedAction, TuiUpKeyPushedAction, TuiWidgetSelectionKeyPushedAction,
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
    EndorsementsRightsWithTimeGet(EndorsementsRightsWithTimeGetAction),
    EndorsementsRightsWithTimeReceived(EndorsementsRightsWithTimeReceivedAction),
    MempoolEndorsementStatsGet(MempoolEndorsementStatsGetAction),
    MempoolEndorsementStatsReceived(MempoolEndorsementStatsReceivedAction),

    CurrentHeadHeaderGet(CurrentHeadHeaderGetAction),
    CurrentHeadHeaderReceived(CurrentHeadHeaderRecievedAction),
    CurrentHeadHeaderChanged(CurrentHeadHeaderChangedAction),
    CycleChanged(CycleChangedAction),
    NetworkConstantsGet(NetworkConstantsGetAction),
    NetworkConstantsReceived(NetworkConstantsReceivedAction),
    CurrentHeadMetadataGet(CurrentHeadMetadataGetAction),
    CurrentHeadMetadataReceived(CurrentHeadMetadataReceivedAction),
    CurrentHeadMetadataChanged(CurrentHeadMetadataChangedAction),
    BestRemoteLevelGet(BestRemoteLevelGetAction),
    BestRemoteLevelReceived(BestRemoteLevelReceivedAction),
    BestRemoteLevelChanged(BestRemoteLevelChangedAction),

    OperationsStatisticsGet(OperationsStatisticsGetAction),
    OperationsStatisticsReceived(OperationsStatisticsReceivedAction),

    ApplicationStatisticsGet(ApplicationStatisticsGetAction),
    ApplicationStatisticsReceived(ApplicationStatisticsReceivedAction),
    PerPeerBlockStatisticsGet(PerPeerBlockStatisticsGetAction),
    PerPeerBlockStatisticsReceived(PerPeerBlockStatisticsReceivedAction),
    BakingRightsReceived(BakingRightsReceivedAction),
    BakingRightsGet(BakingRightsGetAction),

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
