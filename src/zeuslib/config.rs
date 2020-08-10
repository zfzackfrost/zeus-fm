use crate::zeuslib::events::loopaction::EventLoopAction;
use crate::zeuslib::input::KeySequence;
use crate::zeuslib::state::State;
use std::collections::HashMap;
use termion::event::Key;

use crate::zeuslib::actions::*;

pub struct Config {
    pub key_map: HashMap<KeySequence, Box<dyn Fn(&mut State) -> EventLoopAction>>,
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
