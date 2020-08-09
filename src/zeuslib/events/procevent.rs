use crate::zeuslib::state::State;
use crate::zeuslib::input::KeySequence;
use crate::zeuslib::config::Config;
use crate::zeuslib::events::response::EventResponse;


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

fn handle_key_event(state: &mut State, config: &Config, k: Key) -> EventResponse {
    let now = Instant::now();
    if k == Key::Esc {
        state.key_seq.clear();
        state.last_key_time = Some(now);
        return EventResponse::ContinueLoop;
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
            let r = action();
            state.key_seq.clear();
            if r == EventResponse::QuitLoop {
                return r;
            }
        }
    }
    
    EventResponse::ContinueLoop
}

pub fn handle_input(state: &mut State, config: &Config, key: Key) -> EventResponse {
    handle_key_event(state, config, key)
}
