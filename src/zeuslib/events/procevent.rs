use crate::zeuslib::state::State;
use crate::zeuslib::input::KeySequence;
use crate::zeuslib::config::Config;
use crate::zeuslib::events::loopaction::EventLoopAction;


use std::time::{Duration, Instant};
use termion::event::{Key};

lazy_static! {
    static ref QUIT_KEYSEQ: KeySequence = KeySequence::from_keys(vec![Key::Char('q')]);
    static ref NEXTTAB_KEYSEQ: KeySequence =
        KeySequence::from_keys(vec![Key::Char('g'), Key::Char('t'),]);
    static ref NAVUP_KEYSEQ: KeySequence = KeySequence::from_keys(vec![
        Key::Char('k')
    ]);
    static ref NAVDOWN_KEYSEQ: KeySequence = KeySequence::from_keys(vec![
        Key::Char('j')
    ]);
    static ref KEY_TIMEOUT: Duration = Duration::from_millis(1000);
}

fn handle_key_event(mut state: &mut State, config: &Config, k: Key) -> EventLoopAction {
    let now = Instant::now();
    if k == Key::Esc {
        state.key_seq.clear();
        state.last_key_time = Some(now);
        return EventLoopAction::ContinueLoop;
    }
    match state.last_key_time {
        Some(kt) => {
            let elapsed = now.duration_since(kt);
            if elapsed >= *KEY_TIMEOUT {
                state.key_seq.clear();
            }
        }
        _ => {}
    }
    state.last_key_time = Some(now);
    state.key_seq.push(k);

    let key_map = &config.key_map;
    
    for kv in key_map.iter() {
        let seq = kv.0;
        let action = kv.1;
        if state.key_seq == *seq {
            let r = action(&mut state);
            state.key_seq.clear();
            if r == EventLoopAction::QuitLoop {
                return r;
            }
        }
    }
    
    EventLoopAction::ContinueLoop
}

pub fn handle_input(state: &mut State, config: &Config, key: Key) -> EventLoopAction {
    handle_key_event(state, config, key)
}
