use std::rc::Rc;
use std::time::Instant;

pub use self::tabstate::{TabState, PANELS_PER_TAB};
use crate::zeuslib::input::KeySequence;
pub use crate::zeuslib::ui::filelist::{FileList, FileListItem, FileListRc};
use crate::zeuslib::ui::panel::*;

const DEFAULT_PANEL_IDX: usize = 1;

pub struct State {
    pub current_tab: usize,
    pub key_seq: KeySequence,
    pub message: String,
    pub tabs: Vec<TabState>,
    pub last_key_time: Option<Instant>,
    pub current_panel_idx: usize,
}

impl State {
    pub fn default() -> Self {
        Self::from_tab_count(1)
    }

    pub fn from_tab_count(tab_count: u8) -> Self {
        let tabs: Vec<TabState> = (0..(tab_count)).map(|_x| TabState::default()).collect();
        let mut state = Self {
            current_tab: 0,
            key_seq: KeySequence::default(),
            message: String::from(""),
            tabs,
            last_key_time: None,
            current_panel_idx: DEFAULT_PANEL_IDX,
        };
        state.refresh();
        state.select_initial_panel();
        state
    }
    fn select_initial_panel(&mut self) {
        let panel = &mut self.get_current_panel_mut();
        if let Ok(Some(panel)) = panel {
            let mut panel = panel.borrow_mut();
            if !panel.items.is_empty() {
                panel.select(0);
            }
        }
    }
    pub fn refresh(&mut self) {
        let current_panel_idx: usize = self.current_panel_idx;
        let tab = { &mut self.get_current_tab_mut() };
        for i in 0..tab.panels.len() {
            let panel = tab.panels.get_mut(i);
            if let Some(Panel::FileListPanel(Some(panel))) = panel {
                let cursor_pos = {
                    let panel = panel.borrow();
                    panel.cursor_pos
                };
                let mut panel = panel.borrow_mut();
                panel.refresh_list();
                if current_panel_idx == i {
                    panel.select(cursor_pos);
                } else {
                    panel.unselect();
                }
            }
        }
    }
    pub fn get_current_panel(&self) -> Result<FileListRc, ()> {
        let idx: usize = self.current_panel_idx;
        if self.current_panel_idx < PANELS_PER_TAB {
            let tab = &self.get_current_tab();
            let panels = &tab.panels;
            if let Some(Panel::FileListPanel(Some(panel))) = panels.get(idx) {
                return Ok(Some(Rc::clone(panel)));
            }
        }
        Err(())
    }

    pub fn get_current_panel_mut(&mut self) -> Result<FileListRc, ()> {
        let idx: usize = self.current_panel_idx;
        if self.current_panel_idx < PANELS_PER_TAB {
            let tab = &self.get_current_tab();
            let panels = &tab.panels;
            if let Some(Panel::FileListPanel(Some(panel))) = panels.get(idx) {
                return Ok(Some(Rc::clone(panel)));
            }
        }
        Err(())
    }

    pub fn get_current_tab(&self) -> &TabState {
        self.tabs.get(self.current_tab).unwrap()
    }
    pub fn get_current_tab_mut(&mut self) -> &mut TabState {
        self.tabs.get_mut(self.current_tab).unwrap()
    }

    pub fn selected(&self) -> Option<usize> {
        let panel = self.get_current_panel();
        if let Ok(Some(p)) = panel {
            let p = p.borrow_mut();
            return p.selected();
        }
        None
    }

    pub fn selected_mut(&mut self) -> Option<usize> {
        let panel = self.get_current_panel();
        if let Ok(Some(p)) = panel {
            let p = p.borrow_mut();
            return p.selected();
        }
        None
    }

    pub fn next_tab(&mut self) {
        self.current_tab += 1;
        if self.current_tab >= self.tabs.len() {
            self.current_tab = 0;
        }
    }

    pub fn next_panel(&mut self) {
        self.current_panel_idx += 1;
        let tab = self.get_current_tab();
        if self.current_panel_idx >= tab.panels.len() {
            self.current_panel_idx = 0;
        }
    }
}

pub mod tabstate;
