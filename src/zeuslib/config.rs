use std::collections::HashMap;
use crate::zeuslib::input::KeySequence;
use termion::event::Key;
use crate::zeuslib::events::response::EventResponse;

pub struct Config {
    pub key_map: HashMap<KeySequence, Box<dyn Fn() -> EventResponse>>,
}

fn quit_action() -> EventResponse {
    return EventResponse::QuitLoop;
}

impl Config {
    pub fn default() -> Self {
        let mut config = Self {
            key_map: HashMap::new()
        };
        config.map_key(
            KeySequence::from_keys(vec![Key::Char('q')]),
            quit_action
        );
        
        config
    }
    pub fn map_key<F>(&mut self, k: KeySequence, f: F) where 
        F: Fn() -> EventResponse + 'static {
        self.key_map.insert(k, Box::new(f));
    }
}
