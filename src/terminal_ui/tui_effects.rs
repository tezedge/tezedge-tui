use crate::{
    automaton::{Action, ActionWithMeta, Store},
    endorsements::EndorsementsScreen,
    extensions::Renderable,
    operations::StatisticsScreen,
    services::{tui_service::TuiService, Service},
};

use super::ActivePage;

pub fn tui_effects<S>(store: &mut Store<S>, action: &ActionWithMeta)
where
    S: Service,
{
    match &action.action {
        Action::DrawScreen(_) => {
            match store.state().ui.active_page {
                ActivePage::Synchronization => todo!(),
                ActivePage::Mempool => {
                    let state = store.state().clone();
                    // TODO: error handling
                    store
                        .service()
                        .tui()
                        .terminal()
                        .draw(|f| EndorsementsScreen::draw_screen(&state, f));
                }
                ActivePage::Statistics => {
                    let state = store.state().clone();
                    store
                        .service()
                        .tui()
                        .terminal()
                        .draw(|f| StatisticsScreen::draw_screen(&state, f));
                }
            }
        }
        Action::Shutdown(_) => {}
        _ => {}
    }
}
