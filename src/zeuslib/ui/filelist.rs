use tui::widgets::{ListState, ListItem, List, Borders, Block, StatefulWidget};
use tui::{Frame, backend::Backend};
use tui::{text::Span};
use tui::layout::{Rect};
use tui::buffer::{Buffer};
pub use tui::style::{Style, Color, Modifier};

use std::path::Path;

#[derive(Clone)]
#[derive(Debug)]
pub struct FileListItem {
    pub path: String,
    pub marked: bool,
}


impl FileListItem {
    pub fn new(path: String) -> Self {
        Self {
            path,
            marked: false
        }
    }

    pub fn is_dir(&self) -> bool {
        let p = Path::new(&self.path);
        p.is_dir()
    }

    pub fn is_file(&self) -> bool {
        let p = Path::new(&self.path);
        p.is_file()
    }

    fn get_style(&self) -> Style {
        if self.marked {
            Style::default()
                .fg(Color::LightBlue)
        } else {
            Style::default()
                .fg(Color::White)
        }
    }
    fn get_text(&self) -> String {
        let base = if self.is_dir() {
            format!(" [{}]", self.path)  
        } else {
            format!(" {}", self.path)
        };


        if self.marked {
            format!(">{}", base.trim_start())
        } else {
            base
        }
    }

    fn order(a: &FileListItem, b: &FileListItem) -> std::cmp::Ordering {
        let a_dir = a.is_dir();
        let b_dir = b.is_dir();
        if a_dir == b_dir {
            if a.path < b.path {
                std::cmp::Ordering::Less
            } else if a.path > b.path {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        } else if a_dir && !b_dir {
            std::cmp::Ordering::Less
        } else if !a_dir && b_dir {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}



#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
pub struct FileList {
    pub state: ListState,
    pub items: Vec<FileListItem>,
    root: String,
}

impl StatefulWidget for FileList {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    {
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|x| {
                ListItem::new(Span::raw(x.get_text())).style(x.get_style())
            }).collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(self.root.clone()))
            .highlight_style(
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );
        items.render(area, buf, state);
    }
}

#[allow(dead_code)]
impl FileList {
    pub fn new(root: &str) -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
            root: String::from(root),
        }
    }

    pub fn with_items(items: &[FileListItem], root: &str) -> Self {
        Self {
            state: ListState::default(),
            items: Vec::from(items),
            root: String::from(root),
        }
    }

    pub fn set_items(&mut self, items: &[FileListItem]) {
        self.items = Vec::from(items);
    }

    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index));
    }
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn refresh_list(&mut self) {
        let p = Path::new(&self.root);
        if !p.is_dir() {
            return;
        }
        self.items.clear();
        if let Ok(dir) = p.read_dir() {
            for entry in dir {
                if let Ok(entry) = entry {
                    let p = entry.path();
                    if let Some(p) = p.to_str() {
                        self.items.push(FileListItem::new(String::from(p)));
                    }
                }
            }
        }

        self.items.sort_by(FileListItem::order);
    }

    pub fn set_root(&mut self, root: &str) {
        self.root = String::from(root);
        self.refresh_list();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, size: &Rect) {
        f.render_stateful_widget(self.clone(), *size, &mut self.state);
    }
}

