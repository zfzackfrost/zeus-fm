use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::rc::Rc;
use termion::event::Key;
use toml::Value;

use crate::zeuslib::actions::*;
use crate::zeuslib::input::KeySequence;

type KeyMap = HashMap<KeySequence, Action>;

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

    fn process_config_mappings(toml_value: &Value) -> KeyMap {
        let fallback = toml::value::Table::new();
        let mappings = toml_value["mappings"].as_table().unwrap_or(&fallback);
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
                    let key_chars = keyseq.iter().map(|x| {
                        let s = x.as_str();
                        let s = if let Some(s) = s { s } else { "" };
                        let c = s.chars().nth(0).unwrap();
                        Key::Char(c)
                    });
                    Vec::from_iter(key_chars)
                };
                key_map.insert(KeySequence::from_keys(&keyseq), action);
            }
        }
        key_map
    }

    fn process_config_file(value: &Value) -> Self {
        Self {
            key_map: Self::process_config_mappings(&value),
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
        config.map_key(
            KeySequence::from_keys(&[Key::Char('\t')]),
            &actions["next_panel"],
        );
        config
    }
    pub fn map_key(&mut self, k: KeySequence, action: &Action) {
        self.key_map.insert(k, Rc::clone(action));
    }
}
