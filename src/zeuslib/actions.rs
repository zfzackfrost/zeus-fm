use std::collections::HashMap;
use std::rc::Rc;

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

pub type Action = Rc<dyn Fn(&mut State) -> EventLoopAction>;

pub fn get_actions() -> HashMap<String, Action> {
    let mut actions: HashMap<String,Action> = HashMap::new();
    actions.insert(String::from("quit"), Rc::new(quit_action));
    actions.insert(String::from("move_down"), Rc::new(move_down_action));
    actions.insert(String::from("move_up"), Rc::new(move_up_action));
    actions.insert(String::from("mark"), Rc::new(mark_action));
    actions.insert(String::from("next_panel"), Rc::new(next_panel_action));
    actions
}
