use std::time::{Instant};

use crate::zeuslib::input::KeySequence;
use crate::zeuslib::ui::statefullist::{StatefulList};
pub use crate::zeuslib::ui::filelist::{FileListItem};

pub struct State {
    pub current_tab: u8,
    pub tab_count: u8,
    pub key_seq: KeySequence,
    pub message: String,
    pub left_panel: StatefulList<FileListItem>,
    pub right_panel: StatefulList<FileListItem>,
    pub last_key_time: Option<Instant>,
}


impl State {
    pub fn default() -> Self {
        Self::from_tab_count(1)
    }

    pub fn from_tab_count(tab_count: u8) -> Self {
        let state = Self {
            current_tab: 0,
            tab_count,
            key_seq: KeySequence::default(),
            message: String::from(""),
            left_panel: StatefulList::new("Left"),
            right_panel: StatefulList::new("Right"),
            last_key_time: None,
        };
        state
    }

    fn next_tab(&mut self) {
        self.current_tab += 1;
        if self.current_tab >= self.tab_count {
            self.current_tab = 0;
        }
        self.key_seq.clear();
    }

}
