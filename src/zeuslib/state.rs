use std::time::Instant;

use crate::zeuslib::input::KeySequence;
pub use crate::zeuslib::ui::filelist::{FileList, FileListItem};

const NUM_PANELS: usize = 3;
const DEFAULT_PANEL_IDX: usize = 1;


pub struct State {
    pub current_tab: u8,
    pub tab_count: u8,
    pub key_seq: KeySequence,
    pub message: String,
    pub panels: Vec<FileList>,
    pub last_key_time: Option<Instant>,
    pub current_panel_idx: usize,
}

impl State {
    pub fn default() -> Self {
        Self::from_tab_count(1)
    }

    pub fn from_tab_count(tab_count: u8) -> Self {
        let mut state = Self {
            current_tab: 0,
            tab_count,
            key_seq: KeySequence::default(),
            message: String::from(""),
            panels: {
                let work_dir = std::env::current_dir().expect("Could not get working directory!");
                let work_dir = work_dir.to_str().unwrap();

                let v: Vec<FileList> = (0..(NUM_PANELS))
                    .map(|_x| FileList::new(work_dir))
                    .collect();
                v
            },
            last_key_time: None,
            current_panel_idx: DEFAULT_PANEL_IDX,
        };
        state.refresh();
        state.select_initial_panel();
        state
    }
    fn select_initial_panel(&mut self) {
        let panel = &mut self.get_current_panel_mut();
        if let Some(panel) = panel {
            if !panel.items.is_empty() {
                panel.select(0);
            }
        }
    }
    pub fn refresh(&mut self) {
        for i in 0 .. self.panels.len() {
            let panel = self.panels.get_mut(i).unwrap();
            panel.refresh_list();
            if self.current_panel_idx == i {
                panel.select(panel.cursor_pos);
            } else {
                panel.unselect();
            }
        }
    }
    pub fn get_current_panel(&self) -> Option<&FileList> {
        self.panels.get(self.current_panel_idx)
    }

    pub fn get_current_panel_mut(&mut self) -> Option<&mut FileList> {
        self.panels.get_mut(self.current_panel_idx)
    }

    pub fn selected(&self) -> Option<usize> {
        let panel = self.get_current_panel();
        if let Some(p) = panel {
            p.selected()
        } else {
            None
        }
    }

    pub fn selected_mut(&mut self) -> Option<usize> {
        let panel = self.get_current_panel();
        if let Some(p) = panel {
            p.selected()
        } else {
            None
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab += 1;
        if self.current_tab >= self.tab_count {
            self.current_tab = 0;
        }
    }

    pub fn next_panel(&mut self) {
        self.current_panel_idx += 1;
        if self.current_panel_idx >= self.panels.len() {
            self.current_panel_idx = 0;
        }
    }
}
