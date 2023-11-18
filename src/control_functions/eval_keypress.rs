// terminal related
//TODO clean up the match statement (make a lot shorter, more generic??)
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::audio_functions::audio_controls::best_space;
use crate::audio_functions::audio_controls::skip;
use crate::audio_functions::audio_controls::skip_back;
use crate::audio_functions::play_audio_file::play_audio_file;
use crate::file_functions::delete_file::delete_file;
//time
use crate::audio_functions::create_sink::PackagedSink;
use crate::update_statistics;
use crate::get_runtime;

use crate::{graceful_shutdown, Statistics};
use crate::{speed_down, speed_reset, speed_up};
use crate::{volume_down, volume_up};
use std::ops::Index;
use std::time::Duration;

#[allow(clippy::pedantic)] // shut up about the line count, i know, ill fix it later
pub fn eval_keypress(
    mut packed: PackagedSink,
    file_list: Vec<String>,
    index: usize,
    mut stats: Statistics,
) -> (PackagedSink, Vec<String>, usize, Statistics) {
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
                        debug_log!("^c pressed, shutting down.");
                        graceful_shutdown("[eval_keypress] : done!", 0);
                    }
                    // replay (space)
                    KeyEvent {
                        code: KeyCode::Char(' '),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("space pressed, replaying...");
                        packed = best_space(packed, file_list.index(index));
                        (packed, file_list, index, stats)
                    }
                    // volume up (up key)
                    KeyEvent {
                        code: KeyCode::Up,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("up pressed, increasing volume.");
                        volume_up(&packed);
                        (packed, file_list, index, stats)
                    }

                    // volume down (down key)
                    KeyEvent {
                        code: KeyCode::Down,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("down pressed, decreasing volume.");
                        volume_down(&packed);
                        (packed, file_list, index, stats)
                    }

                    // speed up (right key)
                    KeyEvent {
                        code: KeyCode::Right,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("right pressed, increasing speed.");
                        speed_up(&packed);
                        (packed, file_list, index, stats)
                    }

                    // slow down (left key)
                    KeyEvent {
                        code: KeyCode::Left,
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("left pressed, decreasing speed.");
                        speed_down(&packed);
                        (packed, file_list, index, stats)
                    }

                    // reset speed (x)
                    KeyEvent {
                        code: KeyCode::Char('x'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("x pressed, resetting speed.");
                        speed_reset(&packed);
                        (packed, file_list, index, stats)
                    }

                    // previous sound (a)
                    KeyEvent {
                        code: KeyCode::Char('a'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("a pressed, going back.");
                        skip_back(packed, file_list, index, stats)
                    }

                    // next sound (s)
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("s pressed, skipping sound.");
                        skip(packed, file_list, index, stats)
                    }

                    // delete clip (d)
                    KeyEvent {
                        code: KeyCode::Char('d'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: event::KeyEventKind::Press,
                        state: event::KeyEventState::NONE,
                    } => {
                        debug_log!("d pressed, deleting sound.");
                        let new_file_list = delete_file(file_list, index);
                        debug_log!("calculating file length...");
                        let time_change = get_runtime(vec![new_file_list.index(index).to_string()]);
                        // now play the new sound
                        match play_audio_file(std::path::Path::new(new_file_list.index(index)), &mut packed){
                            Ok(_) => {},
                            Err(err) => graceful_shutdown(
                                format!("[eval_keypress] : error playing new file after deleting!: {err:#?}").as_str(),
                                1,
                            ),
                        };
                        // add a deleted clip to the statistics
                        stats = update_statistics(stats, 0, 1, 0.0, -time_change);
                        (packed, new_file_list, index, stats)
                    }

                    _ => {
                        debug_log!("Key without command pressed, ignoring.");
                        (packed, file_list, index, stats)
                    }
                }
                //debug_log!("[main] : keypress : {:?},{:?}\r", event.code, event.kind);
            } else {
                (packed, file_list, index, stats)
            }
        }
        Ok(false) => (packed, file_list, index, stats),
        Err(err) => graceful_shutdown(
            format!("[eval_keypress] : error with reading keypress: {err:#?}").as_str(),
            1,
        ),
    }
}
