use crossterm::event::KeyModifiers;

use crate::{
    automaton::{Action, ActionWithMeta, State},
    extensions::SortableByFocus,
};

use super::{ActivePage, ActiveWidget};

pub fn tui_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::ChangeScreen(action) => {
            state.ui.active_page = action.screen.clone();

            // after we change the screen, we need to set the active widget
            match action.screen {
                ActivePage::Synchronization => state.ui.active_widget = ActiveWidget::PeriodInfo,
                ActivePage::Mempool => state.ui.active_widget = ActiveWidget::EndorserTable,
                ActivePage::Statistics => {
                    state.ui.active_widget = ActiveWidget::StatisticsMainTable
                }
            }
        }
        Action::DrawScreenSuccess(action) => {
            state.ui.screen_width = action.screen_width;

            match state.ui.active_page {
                ActivePage::Synchronization => {}
                ActivePage::Mempool => {
                    let renderable = state
                        .endorsmenents
                        .endorsement_table
                        .renderable_constraints(action.screen_width)
                        .len();
                    state
                        .endorsmenents
                        .endorsement_table
                        .set_rendered(renderable);
                }
                ActivePage::Statistics => {
                    // TODO: we need the size of the table, not the whole screen
                    let renderable = state
                        .operations_statistics
                        .main_operation_statistics_table
                        .renderable_constraints(action.screen_width)
                        .len();
                    state
                        .operations_statistics
                        .main_operation_statistics_table
                        .set_rendered(renderable);
                }
            }
        }
        Action::TuiRightKeyPushed(_) => match state.ui.active_widget {
            ActiveWidget::EndorserTable => state.endorsmenents.endorsement_table.next(),
            ActiveWidget::StatisticsMainTable => state
                .operations_statistics
                .main_operation_statistics_table
                .next(),
            ActiveWidget::StatisticsDetailsTable => state
                .operations_statistics
                .details_operation_statistics_table
                .next(),
            _ => {}
        },
        Action::TuiLeftKeyPushed(_) => match state.ui.active_widget {
            ActiveWidget::EndorserTable => state.endorsmenents.endorsement_table.previous(),
            ActiveWidget::StatisticsMainTable => state
                .operations_statistics
                .main_operation_statistics_table
                .previous(),
            ActiveWidget::StatisticsDetailsTable => state
                .operations_statistics
                .details_operation_statistics_table
                .previous(),
            _ => {}
        },
        Action::TuiDownKeyPushedAction(_) => match state.ui.active_widget {
            ActiveWidget::PeriodInfo => {}
            ActiveWidget::PeerTable => state.synchronization.peer_table_state.select(next_item(
                state.synchronization.peer_metrics.len(),
                state.synchronization.peer_table_state.selected(),
            )),
            ActiveWidget::EndorserTable => state
                .endorsmenents
                .endorsement_table
                .table_state
                .select(next_item(
                    state.endorsmenents.current_head_endorsement_statuses.len(),
                    state.endorsmenents.endorsement_table.table_state.selected(),
                )),
            ActiveWidget::StatisticsMainTable => {
                state
                    .operations_statistics
                    .main_operation_statistics_table
                    .table_state
                    .select(next_item(
                        state
                            .operations_statistics
                            .operations_statistics_sortable
                            .len(),
                        state
                            .operations_statistics
                            .main_operation_statistics_table
                            .table_state
                            .selected(),
                    ));

                if let Some(index) = state
                    .operations_statistics
                    .main_operation_statistics_table
                    .table_state
                    .selected()
                {
                    let hash = state.operations_statistics.operations_statistics_sortable[index]
                        .hash
                        .clone();

                    if let Some(stats) =
                        state.operations_statistics.operations_statistics.get(&hash)
                    {
                        state.operations_statistics.selected_operation_details =
                            Some(stats.to_operations_details());
                    }
                }
            }
            ActiveWidget::StatisticsDetailsTable => state
                .operations_statistics
                .details_operation_statistics_table
                .table_state
                .select(next_item(
                    state
                        .operations_statistics
                        .selected_operation_details
                        .as_ref()
                        .unwrap_or(&Vec::new())
                        .len(),
                    state
                        .operations_statistics
                        .details_operation_statistics_table
                        .table_state
                        .selected(),
                )),
        },
        Action::TuiUpKeyPushedAction(_) => match state.ui.active_widget {
            ActiveWidget::PeriodInfo => {}
            ActiveWidget::PeerTable => {
                state.synchronization.peer_table_state.select(previous_item(
                    state.synchronization.peer_metrics.len(),
                    state.synchronization.peer_table_state.selected(),
                ))
            }
            ActiveWidget::EndorserTable => state
                .endorsmenents
                .endorsement_table
                .table_state
                .select(previous_item(
                    state.endorsmenents.current_head_endorsement_statuses.len(),
                    state.endorsmenents.endorsement_table.table_state.selected(),
                )),
            ActiveWidget::StatisticsMainTable => {
                state
                    .operations_statistics
                    .main_operation_statistics_table
                    .table_state
                    .select(previous_item(
                        state
                            .operations_statistics
                            .operations_statistics_sortable
                            .len(),
                        state
                            .operations_statistics
                            .main_operation_statistics_table
                            .table_state
                            .selected(),
                    ));

                if let Some(index) = state
                    .operations_statistics
                    .main_operation_statistics_table
                    .table_state
                    .selected()
                {
                    let hash = state.operations_statistics.operations_statistics_sortable[index]
                        .hash
                        .clone();

                    if let Some(stats) =
                        state.operations_statistics.operations_statistics.get(&hash)
                    {
                        state.operations_statistics.selected_operation_details =
                            Some(stats.to_operations_details());
                    }
                }
            }
            ActiveWidget::StatisticsDetailsTable => state
                .operations_statistics
                .details_operation_statistics_table
                .table_state
                .select(previous_item(
                    state
                        .operations_statistics
                        .selected_operation_details
                        .as_ref()
                        .unwrap_or(&Vec::new())
                        .len(),
                    state
                        .operations_statistics
                        .details_operation_statistics_table
                        .table_state
                        .selected(),
                )),
        },
        Action::TuiSortKeyPushed(action) => match state.ui.active_widget {
            ActiveWidget::EndorserTable => {}
            ActiveWidget::StatisticsMainTable => {
                state
                    .operations_statistics
                    .operations_statistics_sortable
                    .sort_by_focus(
                        state
                            .operations_statistics
                            .main_operation_statistics_table
                            .selected(),
                        state.delta_toggle,
                    );
                // sort descending
                if let KeyModifiers::CONTROL = action.modifier {
                    state
                        .operations_statistics
                        .operations_statistics_sortable
                        .rev();
                }
            }
            ActiveWidget::StatisticsDetailsTable => {}
            _ => {}
        },
        Action::TuiDeltaToggleKeyPushed(_) => {
            state.delta_toggle = !state.delta_toggle;
        }
        Action::TuiWidgetSelectionKeyPushed(_) => match state.ui.active_page {
            ActivePage::Synchronization => match state.ui.active_widget {
                ActiveWidget::PeriodInfo => state.ui.active_widget = ActiveWidget::PeerTable,
                _ => state.ui.active_widget = ActiveWidget::PeriodInfo,
            },
            ActivePage::Mempool => state.ui.active_widget = ActiveWidget::EndorserTable,
            ActivePage::Statistics => match state.ui.active_widget {
                ActiveWidget::StatisticsMainTable => {
                    state.ui.active_widget = ActiveWidget::StatisticsDetailsTable
                }
                ActiveWidget::StatisticsDetailsTable => {
                    state.ui.active_widget = ActiveWidget::StatisticsMainTable
                }
                _ => state.ui.active_widget = ActiveWidget::StatisticsMainTable,
            },
        },
        Action::Init(_) => {}
        _ => {}
    }
}

pub fn next_item(total: usize, selection_index: Option<usize>) -> Option<usize> {
    match selection_index {
        Some(selection_index) => {
            if total != 0 {
                let next_index = selection_index + 1;
                if next_index > total - 1 {
                    return Some(0);
                } else {
                    return Some(next_index);
                }
            }
            Some(0)
        }
        None => Some(0),
    }
}

pub fn previous_item(total: usize, selection_index: Option<usize>) -> Option<usize> {
    match selection_index {
        Some(selection_index) => {
            if total != 0 {
                if selection_index > 0 {
                    return Some(selection_index - 1);
                } else {
                    return Some(total - 1);
                }
            }
            Some(0)
        }
        None => Some(0),
    }
}
