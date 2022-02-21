use crate::{
    automaton::{Action, ActionWithMeta, Store},
    baking::BakingScreen,
    endorsements::EndorsementsScreen,
    extensions::Renderable,
    operations::StatisticsScreen,
    rpc::RpcRequestAction,
    services::{
        rpc_service::{RpcCall, RpcTarget},
        tui_service::TuiService,
        Service,
    },
    synchronization::SynchronizationScreen,
};

use super::{
    ActivePage, CurrentHeadHeaderChangedAction, CycleChangedAction, DrawScreenSuccessAction,
};

pub fn tui_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    match &action.action {
        Action::DrawScreen(_) => {
            let res = match store.state().ui.active_page {
                ActivePage::Synchronization => {
                    let state = store.state().clone();
                    store
                        .service()
                        .tui()
                        .terminal()
                        .draw(|f| SynchronizationScreen::draw_screen(&state, f))
                }
                ActivePage::Endorsements => {
                    let state = store.state().clone();
                    // TODO: error handling
                    store
                        .service()
                        .tui()
                        .terminal()
                        .draw(|f| EndorsementsScreen::draw_screen(&state, f))
                }
                ActivePage::Statistics => {
                    let state = store.state().clone();
                    store
                        .service()
                        .tui()
                        .terminal()
                        .draw(|f| StatisticsScreen::draw_screen(&state, f))
                }
                ActivePage::Baking => {
                    let state = store.state().clone();
                    store
                        .service()
                        .tui()
                        .terminal()
                        .draw(|f| BakingScreen::draw_screen(&state, f))
                }
            };
            match res {
                Ok(_) => {
                    let width = store.service().tui().terminal().size().unwrap().width;
                    store.dispatch(DrawScreenSuccessAction {
                        screen_width: width,
                    });
                }
                Err(_) => todo!(),
            }
        }
        Action::CurrentHeadHeaderReceived(action) => {
            if store.state().current_head_header.level < action.current_head_header.level {
                store.dispatch(CurrentHeadHeaderChangedAction {
                    current_head_header: action.current_head_header.clone(),
                });
            }

            // TODO: correct cycle info is needed from the block (not header)
            // remove after implemented
            let old_cycle = store.state().current_head_header.level / 4096;
            let new_cycle = action.current_head_header.level / 4096;

            if new_cycle > old_cycle {
                // dispatch cycle changed
                store.dispatch(CycleChangedAction { new_cycle });
            }
        }
        Action::CurrentHeadHeaderGet(_) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(RpcTarget::CurrentHeadHeader, None),
            });
        }
        Action::NetworkConstantsGet(_) => {
            store.dispatch(RpcRequestAction {
                call: RpcCall::new(RpcTarget::NetworkConstants, None),
            });
        }
        Action::Shutdown(_) => {}
        _ => {}
    }
}
