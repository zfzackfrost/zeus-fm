pub use tui::{Frame};
pub use tui::layout::Rect;
pub use crate::zeuslib::Backend;

use tui::widgets::{Widget};

pub trait Drawable {
    fn draw(&mut self, f: &mut Frame<Backend>, size: &Rect); 
}

impl<T: Clone + Widget> Drawable for T {
    fn draw(&mut self, f: &mut Frame<Backend>, size: &Rect) {
        f.render_widget(self.clone(), *size);
    }
}
