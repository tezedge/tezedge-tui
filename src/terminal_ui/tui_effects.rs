use crossterm::{execute, event::DisableMouseCapture, terminal::{LeaveAlternateScreen, disable_raw_mode}};

use crate::{
    automaton::{Action, ActionWithMeta, Store},
    services::{Service, tui_service::TuiService}, endorsements::EndorsementsScreen, extensions::Renderable,
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
                    store.service().tui().terminal().draw(|f| EndorsementsScreen::draw_screen(&state, f));
                },
                ActivePage::Statistics => todo!(),
            }
        }
        Action::Shutdown(_) => {
            let backend_mut = store.service().tui().terminal().backend_mut();
            execute!(
                backend_mut,
                LeaveAlternateScreen,
                DisableMouseCapture
            );
            disable_raw_mode();
            store.service().tui().terminal().show_cursor();
        }
        _ => {}
    }
}
