// terminal related
//TODO clean up the match statement (make a lot shorter, more generic??)
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::audio_functions::audio_controls::best_space;
use crate::audio_functions::audio_controls::skip_back;
use crate::audio_functions::audio_controls::skip;
use crate::audio_functions::play_audio_file::play_audio_file;
use crate::file_functions::delete_file::delete_file;
//time
use crate::audio_functions::create_sink::PackagedSink;
use crate::graceful_shutdown;
use crate::{speed_down, speed_reset, speed_up};
use crate::{volume_down, volume_up};
use std::ops::Index;
use std::time::Duration;

#[allow(clippy::pedantic)] // shut up about the line count, i know, ill fix it later
pub fn eval_keypress(mut packed: PackagedSink, file_list: Vec<String>, index: usize) -> (PackagedSink, Vec<String>, usize) {
    // poll for input for 1 144hz frame
    match event::poll(Duration::from_millis(7)) {
        Ok(true) => {
            if let Ok(Event::Key(event)) = event::read() {
                match event {
                    // check against key binds

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
                        debug_println!("[eval_keypress] : space pressed, replaying...");
                        packed = best_space(packed, file_list.index(index));
                        (packed, file_list, index)
                    }
                    // volume up (up key)
                    KeyEvent {
                        code: KeyCode::Up,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_println!("[eval_keypress] : up pressed, increasing volume.");
                        volume_up(&packed);
                        (packed, file_list, index)
                    }

                    // volume down (down key)
                    KeyEvent {
                        code: KeyCode::Down,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_println!("[eval_keypress] : down pressed, decreasing volume.");
                        volume_down(&packed);
                        (packed, file_list, index)
                    }

                    // speed up (right key)
                    KeyEvent {
                        code: KeyCode::Right,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {

                        debug_println!("[eval_keypress] : right pressed, increasing speed.");
                        speed_up(&packed);
                        (packed, file_list, index)
                    }

                    // slow down (left key)
                    KeyEvent {
                        code: KeyCode::Left,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_println!("[eval_keypress] : left pressed, decreasing speed.");
                        speed_down(&packed);
                        (packed, file_list, index)
                    }

                    // reset speed (x)
                    KeyEvent {
                        code: KeyCode::Char('x'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_println!("[eval_keypress] : x pressed, resetting speed.");
                        speed_reset(&packed);
                        (packed, file_list, index)
                    }

                    // previous sound (a)
                    KeyEvent {
                        code: KeyCode::Char('a'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_println!("[eval_keypress] : a pressed, going back.");
                        skip_back(packed, file_list, index)
                    }

                    // next sound (s)
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_println!("[eval_keypress] : s pressed, skipping sound.");
                        skip(packed, file_list, index)
                    }

                    // delete clip (d)
                    KeyEvent {
                        code: KeyCode::Char('d'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_println!("[eval_keypress] : d pressed, deleting sound.");
                        let new_file_list = delete_file(file_list, index);
                        // now play the new sound
                        match play_audio_file(std::path::Path::new(new_file_list.index(index)), &mut packed){
                            Ok(_) => {},
                            Err(err) => graceful_shutdown(
                                format!("[eval_keypress] : error playing new file after deleting!: {err:#?}").as_str(),
                                1,
                            ),
                        };
                        (packed, new_file_list, index)
                    }

                    _ => {(packed, file_list, index)}
                }
                //debug_println!("[main] : keypress : {:?},{:?}\r", event.code, event.kind);
            } else {(packed, file_list, index)}
        }
        Ok(false) => {(packed, file_list, index)}
        Err(err) => graceful_shutdown(
            format!("[eval_keypress] : error with reading keypress: {err:#?}").as_str(),
            1,
        ),
    }
}