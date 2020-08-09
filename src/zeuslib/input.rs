extern crate termion;

use termion::event::Key;
use std::hash::{Hash, Hasher};

pub struct KeySequence {
    keys: Vec<Key>,
}

impl KeySequence {
    pub fn from_keys(keys: Vec<Key>) -> Self {
        Self { keys }
    }
    pub fn default() -> Self {
        Self { keys: vec![] }
    }
    pub fn push(&mut self, key: Key) {
        self.keys.push(key);
    }
    pub fn clear(&mut self) {
        self.keys.clear();
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Key> {
        self.keys.iter()
    }
}

impl Hash for KeySequence {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let iter = self.keys.iter().enumerate();
        for i in iter {
            i.hash(state);
        }
    }
}

impl PartialEq for KeySequence {
    fn eq(&self, other: &Self) -> bool {
        self.keys.iter().eq(other.keys.iter())
    }
}
impl Eq for KeySequence {}
