use crate::{
    automaton::{Action, ActionWithMeta, Store},
    baking::BakingScreen,
    endorsements::EndorsementsScreen,
    extensions::Renderable,
    operations::StatisticsScreen,
    services::{tui_service::TuiService, Service},
    synchronization::SynchronizationScreen,
};

use super::{ActivePage, DrawScreenSuccessAction};

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
        Action::Shutdown(_) => {}
        _ => {}
    }
}
