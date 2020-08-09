#![crate_name = "zeus"]

#[macro_use]
extern crate lazy_static;

pub mod zeuslib {
    pub mod ui;
    pub mod state;
    pub mod input;
    pub mod events;
    pub mod config;
    pub mod utils;
} /* zeuslib */
