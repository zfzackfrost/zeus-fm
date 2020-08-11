use std::collections::HashMap;
use std::rc::Rc;

use crate::zeuslib::events::loopaction::EventLoopAction;
use crate::zeuslib::state::State;

fn quit_action(_state: &mut State) -> EventLoopAction {
    EventLoopAction::QuitLoop
}

fn next_tab_action(state: &mut State) -> EventLoopAction {
    state.next_tab();
    EventLoopAction::ContinueLoop
}

fn new_tab_action(state: &mut State) -> EventLoopAction {
    state.new_tab();
    EventLoopAction::ContinueLoop
}

fn move_down_action(state: &mut State) -> EventLoopAction {
    let panel = state.get_current_panel_mut();
    if let Ok(Some(panel)) = panel {
        let mut panel = panel.borrow_mut();
        panel.next();
    }
    {
        let tab = state.get_current_tab_mut();
        tab.update_preview();
    }
    EventLoopAction::ContinueLoop
}

fn move_up_action(state: &mut State) -> EventLoopAction {
    let panel = state.get_current_panel_mut();
    if let Ok(Some(panel)) = panel {
        let mut panel = panel.borrow_mut();
        panel.previous();
    }
    {
        let tab = state.get_current_tab_mut();
        tab.update_preview();
    }
    EventLoopAction::ContinueLoop
}

fn mark_action(state: &mut State) -> EventLoopAction {
    let selected = { state.selected() };
    let panel = state.get_current_panel_mut();
    if let Some(i) = selected {
        if let Ok(Some(panel)) = panel {
            let mut panel = panel.borrow_mut();
            let items = &mut panel.items;
            items[i].marked = !items[i].marked;
            panel.next();
        }
    }
    EventLoopAction::ContinueLoop
}
fn cd_parent_action(state: &mut State) -> EventLoopAction {
    let tab = state.get_current_tab_mut();
    tab.cd_parent();
    tab.update_preview();
    EventLoopAction::ContinueLoop
}
fn cd_selected_action(state: &mut State) -> EventLoopAction {
    let tab = state.get_current_tab_mut();
    tab.cd_selected();
    tab.update_preview();
    EventLoopAction::ContinueLoop
}

pub type Action = Rc<dyn Fn(&mut State) -> EventLoopAction>;

pub fn get_actions() -> HashMap<String, Action> {
    let mut actions: HashMap<String, Action> = HashMap::new();
    actions.insert(String::from("quit"), Rc::new(quit_action));
    actions.insert(String::from("next_tab"), Rc::new(next_tab_action));
    actions.insert(String::from("new_tab"), Rc::new(new_tab_action));
    actions.insert(String::from("move_down"), Rc::new(move_down_action));
    actions.insert(String::from("move_up"), Rc::new(move_up_action));
    actions.insert(String::from("mark"), Rc::new(mark_action));
    actions.insert(String::from("cd_parent"), Rc::new(cd_parent_action));
    actions.insert(String::from("cd_selected"), Rc::new(cd_selected_action));
    actions
}
