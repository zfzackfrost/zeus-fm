pub mod filelist;

extern crate termion;
use std::io::{self};

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::symbols::DOT;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Tabs};
use tui::{Frame, Terminal};

use crate::zeuslib::state::State;

#[allow(dead_code)]
struct LayoutRects {
    panels: Vec<Rect>,
    header: Rect,
    footer: Rect,
}

impl LayoutRects {
    fn new<B: Backend>(f: &Frame<B>) -> Self {
        let s = f.size();

        let top_level = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(s);

        let center = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(top_level[1]);

        Self {
            panels: vec![center[0], center[1]],
            header: top_level[0],
            footer: top_level[2],
        }
    }
}

fn make_tab_names(state: &State) -> Vec<String> {
    let tc: u32 = state.tab_count.into();
    let nums = 1u32..(tc + 1u32);
    let nums_vec: Vec<_> = nums.map(u32::from).collect();

    nums_vec.iter().map(|x| format!("Tab {}", x)).collect()
}

fn draw_tabs<B: Backend>(f: &mut Frame<B>, state: &State, layout: &LayoutRects) {
    let titles = make_tab_names(state)
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();
    let tabs = Tabs::new(titles)
        .select(state.current_tab.into())
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(DOT);
    f.render_widget(tabs, layout.header);
}

fn draw_panels<B: Backend>(f: &mut Frame<B>, state: &mut State, layout: &LayoutRects) {
    for p in 0 .. state.panels.len() {
        state.panels[p].draw(f, &layout.panels[p]);
    }
}

pub fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    mut state: &mut State,
) -> Result<(), io::Error> {
    terminal.draw(|f| {
        let layout = LayoutRects::new(f);
        draw_tabs(f, &state, &layout);
        draw_panels(f, &mut state, &layout);
    })
}
