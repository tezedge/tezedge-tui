use slog::info;

use crate::{automaton::{Action, ActionWithMeta, State}, baking::{BlockApplicationSummary, BakingSummary}};

use super::{ActivePage, ActiveWidget};

pub fn tui_reducer(state: &mut State, action: &ActionWithMeta) {
    match &action.action {
        Action::DrawScreen(_) => match state.ui.active_page {
            ActivePage::Endorsements => {
                state.endorsmenents.endorsement_table.highlight_sorting();
            }
            ActivePage::Statistics => {
                state
                    .operations_statistics
                    .main_operation_statistics_table
                    .highlight_sorting();
                state
                    .operations_statistics
                    .details_operation_statistics_table
                    .highlight_sorting();
            }
            ActivePage::Baking => {
                state.baking.baking_table.highlight_sorting();
            }
            _ => { /* No sorting highlights requeired on other screens */ }
        },
        Action::ChangeScreen(action) => {
            state.ui.active_page = action.screen.clone();

            // after we change the screen, we need to set the active widget
            match action.screen {
                ActivePage::Synchronization => state.ui.active_widget = ActiveWidget::PeriodInfo,
                ActivePage::Endorsements => state.ui.active_widget = ActiveWidget::EndorserTable,
                ActivePage::Statistics => {
                    state.ui.active_widget = ActiveWidget::StatisticsMainTable
                }
                ActivePage::Baking => state.ui.active_widget = ActiveWidget::BakingTable,
            }
        }
        Action::DrawScreenSuccess(action) => {
            state.ui.screen_width = action.screen_width;

            match state.ui.active_page {
                ActivePage::Endorsements => {
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
                    let renderable = state
                        .operations_statistics
                        .main_operation_statistics_table
                        .renderable_constraints(action.screen_width / 2)
                        .len();
                    state
                        .operations_statistics
                        .main_operation_statistics_table
                        .set_rendered(renderable);
                    state
                        .operations_statistics
                        .details_operation_statistics_table
                        .set_rendered(renderable);
                }
                ActivePage::Baking => {
                    let renderable = state
                        .baking
                        .baking_table
                        .renderable_constraints((action.screen_width * 60) / 100)
                        .len();
                    state.baking.baking_table.set_rendered(renderable);
                }
                _ => { /* No extended table on other screens */ }
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
            ActiveWidget::BakingTable => state.baking.baking_table.next(),
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
            ActiveWidget::BakingTable => state.baking.baking_table.previous(),
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
                    state.endorsmenents.endorsement_table.content.len(),
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
                            .main_operation_statistics_table
                            .content
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
                    let hash = state
                        .operations_statistics
                        .main_operation_statistics_table
                        .content[index]
                        .hash
                        .clone();

                    if let Some(stats) =
                        state.operations_statistics.operations_statistics.get(&hash)
                    {
                        state
                            .operations_statistics
                            .details_operation_statistics_table
                            .content = stats.to_operations_details();
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
                        .details_operation_statistics_table
                        .content
                        .len(),
                    state
                        .operations_statistics
                        .details_operation_statistics_table
                        .table_state
                        .selected(),
                )),
            ActiveWidget::BakingTable => state.baking.baking_table.table_state.select(next_item(
                state.baking.baking_table.content.len(),
                state.baking.baking_table.table_state.selected(),
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
                    state.endorsmenents.endorsement_table.content.len(),
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
                            .main_operation_statistics_table
                            .content
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
                    let hash = state
                        .operations_statistics
                        .main_operation_statistics_table
                        .content[index]
                        .hash
                        .clone();

                    if let Some(stats) =
                        state.operations_statistics.operations_statistics.get(&hash)
                    {
                        state
                            .operations_statistics
                            .details_operation_statistics_table
                            .content = stats.to_operations_details();
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
                        .details_operation_statistics_table
                        .content
                        .len(),
                    state
                        .operations_statistics
                        .details_operation_statistics_table
                        .table_state
                        .selected(),
                )),
            ActiveWidget::BakingTable => {
                state.baking.baking_table.table_state.select(previous_item(
                    state.baking.baking_table.content.len(),
                    state.baking.baking_table.table_state.selected(),
                ))
            }
        },
        Action::TuiSortKeyPushed(_) => match state.ui.active_widget {
            ActiveWidget::EndorserTable => {
                let selected = state.endorsmenents.endorsement_table.selected();
                let sort_order = state.endorsmenents.endorsement_table.sort_order().switch();
                state
                    .endorsmenents
                    .endorsement_table
                    .set_sort_order(sort_order);

                state
                    .endorsmenents
                    .endorsement_table
                    .set_sorted_by(selected);
                state
                    .endorsmenents
                    .endorsement_table
                    .sort_content(state.delta_toggle);
            }
            ActiveWidget::StatisticsMainTable => {
                let seleceted = state
                    .operations_statistics
                    .main_operation_statistics_table
                    .selected();
                let sort_order = state
                    .operations_statistics
                    .main_operation_statistics_table
                    .sort_order()
                    .switch();
                state
                    .operations_statistics
                    .main_operation_statistics_table
                    .set_sort_order(sort_order);

                state
                    .operations_statistics
                    .main_operation_statistics_table
                    .set_sorted_by(seleceted);

                state
                    .operations_statistics
                    .main_operation_statistics_table
                    .sort_content(state.delta_toggle);
            }
            ActiveWidget::StatisticsDetailsTable => {
                if !state
                    .operations_statistics
                    .details_operation_statistics_table
                    .content
                    .is_empty()
                {
                    let selected = state
                        .operations_statistics
                        .details_operation_statistics_table
                        .selected();
                    let sort_order = state
                        .operations_statistics
                        .details_operation_statistics_table
                        .sort_order()
                        .switch();
                    state
                        .operations_statistics
                        .details_operation_statistics_table
                        .set_sort_order(sort_order);

                    state
                        .operations_statistics
                        .details_operation_statistics_table
                        .set_sorted_by(selected);

                    state
                        .operations_statistics
                        .details_operation_statistics_table
                        .sort_content(state.delta_toggle);
                }
            }
            ActiveWidget::BakingTable => {
                let seleceted = state.baking.baking_table.selected();
                let sort_order = state.baking.baking_table.sort_order().switch();

                state.baking.baking_table.set_sort_order(sort_order);

                state.baking.baking_table.set_sorted_by(seleceted);

                state.baking.baking_table.sort_content(state.delta_toggle);
            }
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
            ActivePage::Endorsements => state.ui.active_widget = ActiveWidget::EndorserTable,
            ActivePage::Statistics => match state.ui.active_widget {
                ActiveWidget::StatisticsMainTable => {
                    state.ui.active_widget = ActiveWidget::StatisticsDetailsTable
                }
                ActiveWidget::StatisticsDetailsTable => {
                    state.ui.active_widget = ActiveWidget::StatisticsMainTable
                }
                _ => state.ui.active_widget = ActiveWidget::StatisticsMainTable,
            },
            ActivePage::Baking => state.ui.active_widget = ActiveWidget::BakingTable,
        },
        Action::CurrentHeadHeaderChanged(action) => {
            // in this context the state.current_head_header is the previous, and state.previous_head_header is the previous of the previous
            // we need to store the last baking data that needs all the updated stats until a new block
            // this block is now in the past so we store it's final summary
            if let Some((baking_level, _)) = state.baking.baking_rights.next_baking(
                state.current_head_header.level,
                &state.current_head_header.timestamp,
                state.network_constants.minimal_block_delay,
            ) {
                if baking_level == state.current_head_header.level {
                    state.baking.last_baked_block_level = Some(state.current_head_header.level);
                    state.baking.last_baked_block_hash =
                        Some(state.current_head_header.hash.clone());

                    let block_application_summary = if let Some(application_statistics) = state
                        .baking
                        .application_statistics
                        .get(&state.current_head_header.hash)
                    {
                        BlockApplicationSummary::from(application_statistics.clone())
                    } else {
                        BlockApplicationSummary::default()
                    };

                    let per_peer = if let Some(last_baked_per_peer_stats) = state
                        .baking
                        .per_peer_block_statistics
                        .get(&state.current_head_header.hash)
                    {
                        last_baked_per_peer_stats.clone()
                    } else {
                        Vec::new()
                    };

                    let summary = BakingSummary::new(
                        baking_level,
                        state.previous_head_header.clone(),
                        block_application_summary,
                        per_peer,
                    );

                    state.baking.last_baking_summary = summary;
                }
            }
            // update the state with new headers
            state.previous_head_header = state.current_head_header.clone();
            state.current_head_header = action.current_head_header.clone();
        }
        Action::NetworkConstantsReceived(action) => {
            state.network_constants = action.constants.clone();
        }
        Action::CurrentHeadMetadataChanged(action) => {
            state.current_head_metadata = action.new_metadata.clone();
        }
        Action::CycleChanged(action) => {
            info!(state.log, "Cleanning up baking rights up until level: {}", action.at_level);
            state.baking.baking_rights.cleanup(&action.at_level);
            // TODO: also cleanup endorsement rights
        }
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
