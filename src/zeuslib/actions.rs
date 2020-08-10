use crate::zeuslib::events::loopaction::EventLoopAction;
use crate::zeuslib::state::State;

pub fn quit_action(_state: &mut State) -> EventLoopAction {
    EventLoopAction::QuitLoop
}

pub fn next_panel_action(state: &mut State) -> EventLoopAction {
    state.next_panel();
    state.refresh();
    EventLoopAction::ContinueLoop
}

pub fn move_down_action(state: &mut State) -> EventLoopAction {
    let panel = state.get_current_panel_mut();
    if let Some(p) = panel {
        p.next();
    }
    EventLoopAction::ContinueLoop
}

pub fn move_up_action(state: &mut State) -> EventLoopAction {
    let panel = state.get_current_panel_mut();
    if let Some(p) = panel {
        p.previous();
    }
    EventLoopAction::ContinueLoop
}

pub fn mark_action(state: &mut State) -> EventLoopAction {
    let selected = { state.selected() };
    let panel = state.get_current_panel_mut();
    if let Some(i) = selected {
        if let Some(p) = panel {
            let items = &mut p.items;
            items[i].marked = !items[i].marked;
            p.next();
        }
    }
    EventLoopAction::ContinueLoop
}

