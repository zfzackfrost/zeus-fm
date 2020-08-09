use crate::zeuslib::ui::statefullist::{StatefulListItem, Style, Color};

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
}

impl StatefulListItem for FileListItem {
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
        if self.marked {
            format!(">> {}", self.path)
        } else {
            self.path.clone()
        }
    }
}
