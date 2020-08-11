pub use crate::zeuslib::ui::filelist::{FileList, FileListRc, Rc, RefCell};
pub use crate::zeuslib::ui::drawable::*;

use tui::widgets::{Block, Borders};

pub enum Panel {
    FileListPanel(FileListRc),
    EmptyPanel,
}

fn draw_empty_panel(f: &mut Frame<Backend>, size: &Rect) {
    let block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(block, *size);
}

impl Drawable for Panel {
    fn draw(&mut self, f: &mut Frame<Backend>, size: &Rect) {
        match self {
            Self::FileListPanel(Some(panel)) => {
                let mut panel = panel.borrow_mut();
                panel.draw(f, size);
            },
            Self::EmptyPanel => {
                draw_empty_panel(f, size);
            },
            _ => {}
        }
    }
}
