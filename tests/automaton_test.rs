
use std::{fs::File, time::SystemTime};

use services_mocked::{ServiceMocked, rpc_service_async::RpcServiceMocked, tui_service::TuiServiceMocked, ws_service::WebsocketServiceMocked};
use tezedge_tui::{extensions::AutomatonDump, automaton::{Store, reducer, effects, Action}};

mod services_mocked;

#[test]
pub fn replay_actions() {
    let file = File::open("automaton_dump.json").expect("Cannot open automaton dump file");
    let data: AutomatonDump = serde_json::from_reader(file).expect("Failed to deserialize dump data");

    let service = ServiceMocked {
        rpc: RpcServiceMocked {},
        tui: TuiServiceMocked::new(),
        ws: WebsocketServiceMocked {},
    };

    let mut store = Store::new(reducer, effects, service, SystemTime::now(), data.init_state);

    for action in data.actions {
        match action.action {
            Action::Init(action) => {
                store.dispatch(action);
            },
            Action::Shutdown(action) => {
                store.dispatch(action);
            },
            Action::RpcRequest(action) => {
                store.dispatch(action);
            },
            Action::RpcResponse(action) => {
                store.dispatch(action);
            },
            Action::RpcResponseRead(action) => {
                store.dispatch(action);
            },
            Action::WebsocketRead(action) => {
                store.dispatch(action);
            },
            Action::WebsocketMessageReceived(action) => {
                store.dispatch(action);
            },
            Action::EndorsementsRightsGet(action) => {
                store.dispatch(action);
            },
            Action::EndorsementsRightsReceived(action) => {
                store.dispatch(action);
            },
            Action::EndorsementsStatusesGet(action) => {
                store.dispatch(action);
            },
            Action::EndorsementsStatusesReceived(action) => {
                store.dispatch(action);
            },
            Action::EndorsementsRightsWithTimeGet(action) => {
                store.dispatch(action);
            },
            Action::EndorsementsRightsWithTimeReceived(action) => {
                store.dispatch(action);
            },
            Action::MempoolEndorsementStatsGet(action) => {
                store.dispatch(action);
            },
            Action::MempoolEndorsementStatsReceived(action) => {
                store.dispatch(action);
            },
            Action::CurrentHeadHeaderGet(action) => {
                store.dispatch(action);
            },
            Action::CurrentHeadHeaderReceived(action) => {
                store.dispatch(action);
            },
            Action::CurrentHeadHeaderChanged(action) => {
                store.dispatch(action);
            },
            Action::CycleChanged(action) => {
                store.dispatch(action);
            },
            Action::NetworkConstantsGet(action) => {
                store.dispatch(action);
            },
            Action::NetworkConstantsReceived(action) => {
                store.dispatch(action);
            },
            Action::CurrentHeadMetadataGet(action) => {
                store.dispatch(action);
            },
            Action::CurrentHeadMetadataReceived(action) => {
                store.dispatch(action);
            },
            Action::CurrentHeadMetadataChanged(action) => {
                store.dispatch(action);
            },
            Action::BestRemoteLevelGet(action) => {
                store.dispatch(action);
            },
            Action::BestRemoteLevelReceived(action) => {
                store.dispatch(action);
            },
            Action::BestRemoteLevelChanged(action) => {
                store.dispatch(action);
            },
            Action::OperationsStatisticsGet(action) => {
                store.dispatch(action);
            },
            Action::OperationsStatisticsReceived(action) => {
                store.dispatch(action);
            },
            Action::ApplicationStatisticsGet(action) => {
                store.dispatch(action);
            },
            Action::ApplicationStatisticsReceived(action) => {
                store.dispatch(action);
            },
            Action::PerPeerBlockStatisticsGet(action) => {
                store.dispatch(action);
            },
            Action::PerPeerBlockStatisticsReceived(action) => {
                store.dispatch(action);
            },
            Action::BakingRightsReceived(action) => {
                store.dispatch(action);
            },
            Action::BakingRightsGet(action) => {
                store.dispatch(action);
            },
            Action::ChangeScreen(action) => {
                store.dispatch(action);
            },
            Action::DrawScreen(action) => {
                store.dispatch(action);
            },
            Action::DrawScreenSuccess(action) => {
                store.dispatch(action);
            },
            Action::DrawScreenFailiure(action) => {
                store.dispatch(action);
            },
            Action::TuiRightKeyPushed(action) => {
                store.dispatch(action);
            },
            Action::TuiLeftKeyPushed(action) => {
                store.dispatch(action);
            },
            Action::TuiUpKeyPushedAction(action) => {
                store.dispatch(action);
            },
            Action::TuiDownKeyPushedAction(action) => {
                store.dispatch(action);
            },
            Action::TuiSortKeyPushed(action) => {
                store.dispatch(action);
            },
            Action::TuiDeltaToggleKeyPushed(action) => {
                store.dispatch(action);
            },
            Action::TuiWidgetSelectionKeyPushed(action) => {
                store.dispatch(action);
            },
        }
    }
    // serialize then compare
    let result = serde_json::to_string(store.state()).expect("Failed to deserialize result");
    let expected = serde_json::to_string(&data.end_state).expect("Failed to deserialize expected");
    assert_eq!(result, expected);
}