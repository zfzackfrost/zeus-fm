use crate::zeuslib::events::loopaction::EventLoopAction;
use crate::zeuslib::input::KeySequence;
use crate::zeuslib::state::State;
use std::collections::HashMap;
use termion::event::Key;

pub struct Config {
    pub key_map: HashMap<KeySequence, Box<dyn Fn(&mut State) -> EventLoopAction>>,
}

fn quit_action(_state: &mut State) -> EventLoopAction {
    EventLoopAction::QuitLoop
}

fn next_panel_action(state: &mut State) -> EventLoopAction {
    state.next_panel();
    state.refresh();
    EventLoopAction::ContinueLoop
}

fn move_down_action(state: &mut State) -> EventLoopAction {
    let panel = state.get_current_panel_mut();
    if let Some(p) = panel {
        p.next();
    }
    EventLoopAction::ContinueLoop
}

fn move_up_action(state: &mut State) -> EventLoopAction {
    let panel = state.get_current_panel_mut();
    if let Some(p) = panel {
        p.previous();
    }
    EventLoopAction::ContinueLoop
}

fn mark_action(state: &mut State) -> EventLoopAction {
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

impl Config {
    pub fn default() -> Self {
        let mut config = Self {
            key_map: HashMap::new(),
        };
        config.map_key(KeySequence::from_keys(&[Key::Char('q')]), quit_action);
        config.map_key(KeySequence::from_keys(&[Key::Char('j')]), move_down_action);
        config.map_key(KeySequence::from_keys(&[Key::Char('k')]), move_up_action);
        config.map_key(KeySequence::from_keys(&[Key::Char(' ')]), mark_action);
        config.map_key(
            KeySequence::from_keys(&[Key::Char('\t')]),
            next_panel_action,
        );
        config
    }
    pub fn map_key<F>(&mut self, k: KeySequence, f: F)
    where
        F: Fn(&mut State) -> EventLoopAction + 'static,
    {
        self.key_map.insert(k, Box::new(f));
    }
}
