extern crate termion;
use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::Terminal;
// use tui::layout::{Layout, Constraint, Direction};


use zeus_fm::zeuslib::ui::draw;
use zeus_fm::zeuslib::config::Config;
use zeus_fm::zeuslib::utils::fs::*;
use zeus_fm::zeuslib::state::{State};
use zeus_fm::zeuslib::events::{Events, Event};
use zeus_fm::zeuslib::events::procevent::{handle_input};
use zeus_fm::zeuslib::events::loopaction::{EventLoopAction};

fn main() -> Result<(), Box<dyn Error>> {
    let cfg_path = &*CONFIG_FILE;
    let cfg_dir = &*CONFIG_DIR;
    if let Some(cfg_dir) = cfg_dir {
        if !cfg_dir.is_dir() {
            std::fs::create_dir_all(cfg_dir.to_owned())?;
        }
    }
    
    let config: Config;
    if let Some(cfg_path) = cfg_path {
        if !cfg_path.is_file() {
            std::fs::write(cfg_path.to_owned(), "")?;
        }
        config = if let Some(cfg) = Config::from_file(cfg_path) {
            cfg
        } else {
            Config::default()
        }
    } else {
        config = Config::default();
    }

    let mut state = State::default();

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

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
