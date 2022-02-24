use std::{fs::File, time::SystemTime};
use pretty_assertions::assert_eq;

use services_mocked::{
    rpc_service_async::RpcServiceMocked, tui_service::TuiServiceMocked,
    ws_service::WebsocketServiceMocked, ServiceMocked,
};
use tezedge_tui::{
    automaton::{effects, reducer, Action, Store},
    extensions::AutomatonDump, services::tui_service::TuiService,
};

mod services_mocked;

#[test]
pub fn replay_actions() {
    let file = File::open("automaton_dump.json").expect("Cannot open automaton dump file");
    let data: AutomatonDump =
        serde_json::from_reader(file).expect("Failed to deserialize dump data");

    let service = ServiceMocked {
        rpc: RpcServiceMocked {},
        tui: TuiServiceMocked::new(),
        ws: WebsocketServiceMocked {},
    };

    let mut store = Store::new(
        reducer,
        effects,
        service,
        SystemTime::now(),
        data.init_state,
    );
    let len = data.actions.len();
    for (num, action) in data.actions.into_iter().enumerate() {
        println!(
            "Sending action {} - {}/{}",
            action.action.kind(),
            num + 1,
            len
        );
        match action.action {
            Action::Init(action) => {
                store.dispatch(action);
            }
            Action::Shutdown(action) => {
                store.dispatch(action);
            }
            Action::RpcRequest(action) => {
                store.dispatch(action);
            }
            Action::RpcResponse(action) => {
                store.dispatch(action);
            }
            Action::RpcResponseRead(action) => {
                store.dispatch(action);
            }
            // Action::WebsocketRead(action) => {
            //     store.dispatch(action);
            // },
            Action::WebsocketMessageReceived(action) => {
                store.dispatch(action);
            }
            Action::EndorsementsRightsGet(action) => {
                store.dispatch(action);
            }
            Action::EndorsementsRightsReceived(action) => {
                store.dispatch(action);
            }
            Action::EndorsementsStatusesGet(action) => {
                store.dispatch(action);
            }
            Action::EndorsementsStatusesReceived(action) => {
                store.dispatch(action);
            }
            Action::EndorsementsRightsWithTimeGet(action) => {
                store.dispatch(action);
            }
            Action::EndorsementsRightsWithTimeReceived(action) => {
                store.dispatch(action);
            }
            Action::MempoolEndorsementStatsGet(action) => {
                store.dispatch(action);
            }
            Action::MempoolEndorsementStatsReceived(action) => {
                store.dispatch(action);
            }
            Action::CurrentHeadHeaderGet(action) => {
                store.dispatch(action);
            }
            Action::CurrentHeadHeaderReceived(action) => {
                store.dispatch(action);
            }
            Action::CurrentHeadHeaderChanged(action) => {
                store.dispatch(action);
            }
            Action::CycleChanged(action) => {
                store.dispatch(action);
            }
            Action::NetworkConstantsGet(action) => {
                store.dispatch(action);
            }
            Action::NetworkConstantsReceived(action) => {
                store.dispatch(action);
            }
            Action::CurrentHeadMetadataGet(action) => {
                store.dispatch(action);
            }
            Action::CurrentHeadMetadataReceived(action) => {
                store.dispatch(action);
            }
            Action::CurrentHeadMetadataChanged(action) => {
                store.dispatch(action);
            }
            Action::BestRemoteLevelGet(action) => {
                store.dispatch(action);
            }
            Action::BestRemoteLevelReceived(action) => {
                store.dispatch(action);
            }
            Action::BestRemoteLevelChanged(action) => {
                store.dispatch(action);
            }
            Action::OperationsStatisticsGet(action) => {
                store.dispatch(action);
            }
            Action::OperationsStatisticsReceived(action) => {
                store.dispatch(action);
            }
            Action::ApplicationStatisticsGet(action) => {
                store.dispatch(action);
            }
            Action::ApplicationStatisticsReceived(action) => {
                store.dispatch(action);
            }
            Action::PerPeerBlockStatisticsGet(action) => {
                store.dispatch(action);
            }
            Action::PerPeerBlockStatisticsReceived(action) => {
                store.dispatch(action);
            }
            Action::BakingRightsReceived(action) => {
                store.dispatch(action);
            }
            Action::BakingRightsGet(action) => {
                store.dispatch(action);
            }
            Action::ChangeScreen(action) => {
                store.dispatch(action);
            }
            Action::DrawScreen(action) => {
                store.dispatch(action);
            }
            Action::DrawScreenSuccess(action) => {
                store.dispatch(action);
            }
            Action::DrawScreenFailiure(action) => {
                store.dispatch(action);
            }
            Action::TuiRightKeyPushed(action) => {
                store.dispatch(action);
            }
            Action::TuiLeftKeyPushed(action) => {
                store.dispatch(action);
            }
            Action::TuiUpKeyPushedAction(action) => {
                store.dispatch(action);
            }
            Action::TuiDownKeyPushedAction(action) => {
                store.dispatch(action);
            }
            Action::TuiSortKeyPushed(action) => {
                store.dispatch(action);
            }
            Action::TuiDeltaToggleKeyPushed(action) => {
                store.dispatch(action);
            }
            Action::TuiWidgetSelectionKeyPushed(action) => {
                store.dispatch(action);
            }
            _ => {}
        }
    }
    // serialize then compare
    // let result = serde_json::to_string(store.state()).expect("Failed to serialize result");
    // let expected = serde_json::to_string(&data.end_state).expect("Failed to serialize expected");
    store.service().tui.restore_terminal();
    let resulting_state = store.state().clone();
    
    assert_eq!(resulting_state.network_constants, data.end_state.network_constants);
    println!("Network constants OK");

    assert_eq!(resulting_state.last_applied_level, data.end_state.last_applied_level);
    println!("Last applied level OK");

    assert_eq!(resulting_state.current_head_header, data.end_state.current_head_header);
    println!("Current head header OK");

    assert_eq!(resulting_state.current_head_metadata, data.end_state.current_head_metadata);
    println!("Current head metadata OK");

    // TODO: not equal, investigate
    // assert_eq!(resulting_state.previous_head_header, data.end_state.previous_head_header);
    // println!("Previous head OK");

    assert_eq!(resulting_state.best_remote_level, data.end_state.best_remote_level);
    println!("Remote level OK");

    assert_eq!(resulting_state.baker_address, data.end_state.baker_address);
    println!("Baker address OK");

    assert_eq!(resulting_state.baking, data.end_state.baking);
    println!("BBaking state OK");

}
