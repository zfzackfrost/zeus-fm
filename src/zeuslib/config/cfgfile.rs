use crate::zeuslib::actions::*;
use crate::zeuslib::config::KeyMap;
use crate::zeuslib::input::KeySequence;

use std::iter::FromIterator;

use std::rc::Rc;
use termion::event::Key;

use regex::Regex;
use toml::Value;

lazy_static! {
    static ref MODIFIED_KEY_RE: Regex = Regex::new("(C|c|M|m)-(.)").unwrap();
    static ref SIMPLE_KEY_RE: Regex = Regex::new("(.)").unwrap();
}

fn parse_key_string(key_str: &str) -> Option<Key> {
    if let Some(caps) = (*MODIFIED_KEY_RE).captures(key_str) {
        let modifier = caps[1].to_string().to_uppercase();
        let c = &caps[2].chars().nth(0).unwrap();

        if modifier == "C" {
            Some(Key::Ctrl(*c))
        } else if modifier == "M" {
            Some(Key::Alt(*c))
        } else {
            None
        }
    } else if (*SIMPLE_KEY_RE).is_match(key_str) {
        let c = key_str.chars().nth(0).unwrap();
        Some(Key::Char(c))
    } else {
        None
    }
}

pub fn process_config_mappings(toml_value: &Value) -> KeyMap {
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
                let key_chars = keyseq
                    .iter()
                    .map(|x| {
                        let s = x.as_str();
                        let s = if let Some(s) = s { s } else { "" };
                        parse_key_string(&s)
                    })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap());
                Vec::from_iter(key_chars)
            };
            key_map.insert(KeySequence::from_keys(&keyseq), action);
        }
    }
    key_map
}
