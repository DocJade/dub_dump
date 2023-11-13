// TODO
// Playing audio files
// Progress bar for total progress
// Progress bar for currently playing audio
// Timeline based on clip length
// Volume control
// Speed controls
// Pause
// Play
// Delete

// Make Clippy angry
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::correctness,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::cognitive_complexity,
    clippy::double_parens,
    clippy::len_zero,
    clippy::question_mark,
    clippy::suspicious,
    clippy::todo,
    //clippy::all  //for extra anger
)]

// access to file system, threads, and time
use std::path::Path;

// rodio is our audio player
// use rodio::Sink;

// tui-rs is our terminal interface
// use tui::*;

//public debug 
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub mod audio_functions;
pub mod helper_functions;
use crate::helper_functions::graceful_shutdown::graceful_shutdown;
use crate::audio_functions::play_audio_file::play_audio_file;
use crate::audio_functions::create_sink::{create_sink, PackagedSink};

fn main() {
    // create our audio sink
    let mut packed: PackagedSink = match create_sink() {
        Ok(ok) => ok,
        Err(err) => graceful_shutdown(format!("[main] : error the audio sink and stream: {err:#?}").as_str(), 1),
    };

    // play a sound
    match play_audio_file(Path::new("test.wav"), &mut packed) {
        Ok(_) => {}, // swag, move on.
        Err(err) => graceful_shutdown(format!("[main] : error playing sound: {err:#?}").as_str(), 1)
    }

    // close program when sound is done
    debug_println!("[main] : waiting for sound to finish playing...");
    packed.sink.sleep_until_end();
    debug_println!("[main] : sound has finished playing.");
    graceful_shutdown("[main] : done!", 0)
}
