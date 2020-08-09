extern crate termion;
use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::Terminal;
// use tui::layout::{Layout, Constraint, Direction};


use zeus::zeuslib::ui::draw;
use zeus::zeuslib::config::Config;
use zeus::zeuslib::state::{State};
use zeus::zeuslib::events::{Events, Event};
use zeus::zeuslib::events::procevent::{handle_input};
use zeus::zeuslib::events::loopaction::{EventLoopAction};

fn main() -> Result<(), Box<dyn Error>> {

    let mut state = State::default();

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    let config: Config = Config::default();

    terminal.clear()?;

    loop {
        draw(&mut terminal, &mut state)?;
        if let Event::Input(input) = events.next()? {
            match handle_input(&mut state, &config, input) {
                EventLoopAction::QuitLoop => break,
                _ => {}
            }
        }
    }
    
    terminal.clear()?;

    Result::Ok(())
}
