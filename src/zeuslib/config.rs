use std::collections::HashMap;
use std::rc::Rc;
use std::iter::FromIterator;
use termion::event::Key;
use toml::{Value};
use std::path::{PathBuf};
use std::fs;

use crate::zeuslib::actions::*;
use crate::zeuslib::events::loopaction::EventLoopAction;
use crate::zeuslib::input::KeySequence;
use crate::zeuslib::state::State;

type KeyMap = HashMap<KeySequence, Action>;

pub struct Config {
    pub key_map: KeyMap,
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Option<Self> {
        if !path.is_file() {
            return None
        }
        let value: Option<String> = {
            let contents = fs::read_to_string(path);
            if let Ok(contents) = contents {
                Some(contents)
            } else {
                None
            }
        };
        if let Some(value) = value {
            if let Ok(value) = value.parse::<Value>() {
                return Some(Self::process_config_file(&value));
            }
        }

        None
    }
    
    fn process_config_file(value: &Value) -> Self {
        let fallback = toml::value::Table::new();
        let mappings = value["mappings"].as_table().unwrap_or(&fallback);
        let mut key_map: KeyMap = KeyMap::new();
        let actions = get_actions();
        for (action, keys) in mappings {
            if !actions.contains_key(action) {
                continue;
            }

            let action = Rc::clone(&actions[action]);

            let keyseq = keys.as_array();
            if let Some(keyseq) = keyseq {
                let keyseq = {
                    let key_chars = keyseq.iter().map(|x|{
                        let s = x.as_str();
                        let s = if let Some(s) = s {
                            s
                        } else {
                            ""
                        };
                        let c = s.chars().nth(0).unwrap();
                        Key::Char(c)
                    });
                    Vec::from_iter(key_chars)
                };
                key_map.insert(KeySequence::from_keys(&keyseq), action);
            }
        }
        Self {
            key_map,
        }
    }

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
        self.key_map.insert(k, Rc::new(f));
    }
}
