use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use termion::event::Key;
use toml::Value;

use crate::zeuslib::actions::*;
use crate::zeuslib::input::KeySequence;

use crate::zeuslib::config::KeyMap;
use crate::zeuslib::config::cfgfile::*;



pub struct Config {
    pub key_map: KeyMap,
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Option<Self> {
        if !path.is_file() {
            return None;
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
        Self {
            key_map: process_config_mappings(&value),
        }
    }

    pub fn default() -> Self {
        let mut config = Self {
            key_map: HashMap::new(),
        };
        let actions = get_actions();
        config.map_key(KeySequence::from_keys(&[Key::Char('q')]), &actions["quit"]);
        config.map_key(
            KeySequence::from_keys(&[Key::Char('j')]),
            &actions["move_down"],
        );
        config.map_key(
            KeySequence::from_keys(&[Key::Char('k')]),
            &actions["move_up"],
        );
        config.map_key(KeySequence::from_keys(&[Key::Char(' ')]), &actions["mark"]);
        config
    }
    pub fn map_key(&mut self, k: KeySequence, action: &Action) {
        self.key_map.insert(k, Rc::clone(action));
    }
}
