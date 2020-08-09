extern crate termion;

use std::hash::{Hash, Hasher};
use termion::event::Key;

pub struct KeySequence {
    keys: Vec<Key>,
}

impl KeySequence {
    pub fn from_keys(keys: &[Key]) -> Self {
        Self {
            keys: Vec::from(keys),
        }
    }
    pub fn default() -> Self {
        Self { keys: Vec::new() }
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
        for i in self.keys.iter().enumerate() {
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
