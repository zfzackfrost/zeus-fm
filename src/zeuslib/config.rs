mod cfgfile;
mod cfgdata;

use std::collections::HashMap;

use crate::zeuslib::actions::*;
use crate::zeuslib::input::KeySequence;

pub use self::cfgdata::*;
pub type KeyMap = HashMap<KeySequence, Action>;
