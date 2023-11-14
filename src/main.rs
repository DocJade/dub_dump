// TODO in descending order of priority
// Pause
// Play
// make playing when file is over restart the file
// Delete
// Volume control
// Speed controls
// get dir for files
// Copy all clips into a new folder upon startup (non destructive editing ftw)
// Progress bar for currently playing audio
// Progress bar for total progress
// Timeline based on clip length
// audio clip waveform (not animated)
// logo shimmer

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

//public debug
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub mod audio_functions;
pub mod control_functions;
pub mod helper_functions;
pub mod terminal_functions;
use crate::audio_functions::audio_controls::{
    speed_down, speed_reset, speed_up, volume_down, volume_up,
};
use crate::audio_functions::create_sink::{create_sink, PackagedSink};
use crate::control_functions::eval_keypress::eval_keypress;
//use crate::audio_functions::play_audio_file::play_audio_file;
use crate::helper_functions::graceful_shutdown::graceful_shutdown;
use crate::terminal_functions::set_size::set_size;
use crate::terminal_functions::terminal_setup::terminal_setup;

fn main() {
    // terminal cleanup on shutdown
    let _clean_up = crate::terminal_functions::terminal_setup::CleanUp;

    // do terminal setup

    match terminal_setup() {
        Ok(_) => {
            debug_println!("[main] : Terminal ready!");
        }
        Err(err) => graceful_shutdown(
            format!("[main] : error setting up the terminal.: {err:#?}").as_str(),
            1,
        ),
    }

    // create our audio sink
    let packed: PackagedSink = match create_sink() {
        Ok(ok) => ok,
        Err(err) => graceful_shutdown(
            format!("[main] : Error creating the audio sink and stream: {err:#?}").as_str(),
            1,
        ),
    };

    // now we shall enter the main loop

    loop {
        // check terminal size

        match set_size() {
            Ok(_) => {}
            Err(err) => graceful_shutdown(
                format!("[main] : error with terminal sizing: {err:#?}").as_str(),
                1,
            ),
        };

        // Check for input
        eval_keypress(&packed);
    }
}
