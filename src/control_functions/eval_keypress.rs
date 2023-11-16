// terminal related
//TODO clean up the match statement
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};

//time
use crate::audio_functions::create_sink::PackagedSink;
use crate::graceful_shutdown;
use crate::{speed_down, speed_reset, speed_up};
use crate::{volume_down, volume_up};
use std::time::Duration;

pub fn eval_keypress(packed: &PackagedSink) {
    // poll for input for 1 144hz frame
    match event::poll(Duration::from_millis(7)) {
        Ok(true) => {
            if let Ok(Event::Key(event)) = event::read() {
                match event {
                    // check against keybinds

                    // quit app (^c)
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        //quit
                        debug_println!("[eval_keypress] : ^c pressed, shutting down.");
                        graceful_shutdown("[eval_keypress] : done!", 0);
                    }
                    // replay (space)
                    KeyEvent {
                        code: KeyCode::Char(' '),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        //quit
                        debug_println!("[eval_keypress] : space pressed, replaying...");
                        unimplemented!("unimplemented");
                    }
                    // volume up (up key)
                    KeyEvent {
                        code: KeyCode::Up,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        //quit
                        debug_println!("[eval_keypress] : up pressed, increasing volume.");
                        volume_up(packed);
                    }

                    // volume down (down key)
                    KeyEvent {
                        code: KeyCode::Down,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        //quit
                        debug_println!("[eval_keypress] : down pressed, decreasing volume.");
                        volume_down(packed);
                    }

                    // speed up (right key)
                    KeyEvent {
                        code: KeyCode::Right,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        //quit
                        debug_println!("[eval_keypress] : right pressed, increasing speed.");
                        speed_up(packed);
                    }

                    // slow down (left key)
                    KeyEvent {
                        code: KeyCode::Left,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        //quit
                        debug_println!("[eval_keypress] : left pressed, decreasing speed.");
                        speed_down(packed);
                    }

                    // reset speed (x)
                    KeyEvent {
                        code: KeyCode::Char('x'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        //quit
                        debug_println!("[eval_keypress] : x pressed, resetting speed.");
                        speed_reset(packed);
                    }

                    _ => { /*unimplemented */ }
                }
                //debug_println!("[main] : keypress : {:?},{:?}\r", event.code, event.kind);
            }
        }
        Ok(false) => { /* no pressed keys */ }
        Err(err) => graceful_shutdown(
            format!("[eval_keypress] : error with reading keypress: {err:#?}").as_str(),
            1,
        ),
    }
}
