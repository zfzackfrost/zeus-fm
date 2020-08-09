use tui::widgets::{ListState, ListItem, List, Borders, Block};
use tui::{Frame, backend::Backend};
use tui::{text::Span};
use tui::layout::{Rect};

pub use tui::style::{Style, Color, Modifier};

pub trait StatefulListItem {
    fn get_style(&self) -> Style;
    fn get_text(&self) -> String;
}

#[allow(dead_code)]
pub struct StatefulList<T: StatefulListItem> {
    pub state: ListState,
    pub items: Vec<T>,
    title: String,
}

#[allow(dead_code)]
impl<T: StatefulListItem> StatefulList<T> {
    pub fn new(title: &str) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
            title: String::from(title),
        }
    }

    pub fn with_items(items: Vec<T>, title: &str) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            title: String::from(title),
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
    }

    pub fn select(&mut self, index: usize) {
        self.state.select(Some(index));
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
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|x| {
                ListItem::new(Span::raw(x.get_text())).style(x.get_style())
            }).collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(self.title.clone()))
            .highlight_style(
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );
        f.render_stateful_widget(items, *size, &mut self.state);
    }
}
