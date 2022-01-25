use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

use crate::{
    automaton::{Action, ActionWithMeta, Store},
    endorsements::EndorsementsScreen,
    extensions::Renderable,
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
                ActivePage::Statistics => todo!(),
            }
        }
        Action::Shutdown(_) => {
            let backend_mut = store.service().tui().terminal().backend_mut();
            execute!(backend_mut, LeaveAlternateScreen, DisableMouseCapture);
            disable_raw_mode();
            store.service().tui().terminal().show_cursor();
        }
        _ => {}
    }
}
