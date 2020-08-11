use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use tui::buffer::Buffer;
use tui::layout::Alignment;
use tui::layout::Rect;
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph, Widget};

#[derive(Clone)]
pub struct Preview {
    pub path: Option<PathBuf>,
    file_contents: Option<String>,
}

impl Preview {
    pub fn new(path: Option<PathBuf>) -> Self {
        let file_contents: Option<String> = {
            if let Some(path) = &path {
                if path.is_file() {
                    if let Ok(s) = std::fs::read_to_string(&path) {
                        Some(s)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        Self {
            path,
            file_contents,
        }
    }

    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.path = path.clone();
        self.file_contents = {
            if let Some(path) = &path {
                if path.is_file() {
                    if let Ok(s) = std::fs::read_to_string(&path) {
                        Some(s)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
    }
}

impl Default for Preview {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Widget for Preview {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Preview");
        if let Some(file_contents) = self.file_contents {
            let text = Text::from(file_contents.as_str());
            let text = Paragraph::new(text).block(block.clone());
            text.render(area, buf);
        } else {
            let text = Text::from("No file selected.");
            let text = Paragraph::new(text)
                .block(block.clone())
                .alignment(Alignment::Center);
            text.render(area, buf);
        }
    }
}

pub type PreviewRc = Rc<RefCell<Preview>>;
