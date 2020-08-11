use std::collections::HashSet;
use std::path::PathBuf;

use crate::zeuslib::ui::filelist::{FileList, Rc, RefCell};
use crate::zeuslib::ui::panel::*;

pub const PANELS_PER_TAB: usize = 3;
const MAIN_PANEL_IDX: usize = 1;

pub struct TabState {
    pub dir: Option<PathBuf>,
        pub panels: Vec<Panel>,
    pub marked_paths: HashSet<PathBuf>,
}

impl Default for TabState {
    fn default() -> Self {
        let dir = std::env::current_dir().expect("Failed to find current directory");
        let panels: [Panel; PANELS_PER_TAB] = {
            let left = if let Some(p) = dir.parent() {
                Panel::FileListPanel(Some(Rc::new(RefCell::new(FileList::new(p.to_str().unwrap())))))
            } else {
                Panel::EmptyPanel
            };
            let center = Some(Rc::new(RefCell::new(FileList::new(dir.to_str().unwrap()))));
            let right = Rc::new(RefCell::new(Preview::default()));
            [left, Panel::FileListPanel(center), Panel::PreviewPanel(right)]
        };
        Self {
            dir: Some(dir),
            panels: Vec::from(panels),
            marked_paths: HashSet::new(),
        }
    }
}

impl TabState {
    pub fn cd(&mut self, new_dir: Option<PathBuf>) {
        self.dir = new_dir.clone();
        if let Some(new_dir) = new_dir {
            let mut left_empty = false; // Should the left panel be set to empty?
            {
                if let Panel::FileListPanel(Some(left)) = &self.panels[0] {
                    let mut panel = left.borrow_mut();
                    let parent = new_dir.parent();
                    if let Some(parent) = parent {
                        panel.set_root(parent.to_str().expect("Invalid path!"));
                        panel.refresh_list();
                    } else {
                        left_empty = true;
                    }
                }
            }
            {
                // Needs to be done in a new scope due to borrowing rules
                if left_empty {
                    self.panels[0] = Panel::EmptyPanel;
                }
            }
            {
                if let Panel::FileListPanel(Some(center)) = &self.panels[MAIN_PANEL_IDX] {
                    let mut panel = center.borrow_mut();
                    let pos = panel.cursor_pos;
                    panel.set_root(new_dir.to_str().expect("Invalid path!"));
                    panel.refresh_list();
                    panel.select(pos);
                }
            }
        }
    }

    pub fn cd_parent(&mut self) {
        let new_dir: Option<PathBuf> = if let Some(dir) = &self.dir {
            let parent = dir.parent();
            if let Some(parent) = parent {
                Some(parent.to_path_buf())
            } else {
                None
            }
        } else {
            None
        };
        self.cd(new_dir);
    }
    

    pub fn cd_selected(&mut self) {
        let mut path: Option<PathBuf> = None;
        if let Panel::FileListPanel(Some(panel)) = &mut self.panels[MAIN_PANEL_IDX] {
            let panel = panel.borrow();
            let item = panel.selected_item();
            if let Some(item) = item {
                if !item.is_dir() {
                    return
                }
                path = Some(PathBuf::from(item.path));
            }
        }
        {
            if let Some(path) = path {
                self.cd(Some(path));
            }
        }
    }

    pub fn update_preview(&mut self) {
        if let Panel::PreviewPanel(preview) = &self.panels[2] {
            let mut preview = preview.borrow_mut();
            if let Panel::FileListPanel(Some(main_panel)) = &self.panels[MAIN_PANEL_IDX] {
                let main_panel = main_panel.borrow();
                let selected_item = main_panel.selected_item();
                if let Some(selected_item) = selected_item {
                    let p = selected_item.path;
                    preview.set_path(Some(PathBuf::from(p)));
                }
            }
        }
    }
}
