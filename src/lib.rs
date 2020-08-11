#![crate_name = "zeus_fm"]

#[macro_use]
extern crate lazy_static;




pub mod zeuslib {
    use termion::{input::MouseTerminal, screen::AlternateScreen};
    use tui::backend::TermionBackend;

    pub type Backend = TermionBackend<AlternateScreen<MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>>>;

    pub mod actions;
    pub mod ui;
    pub mod state;
    pub mod input;
    pub mod events;
    pub mod config;
    pub mod utils;
} /* zeuslib */
