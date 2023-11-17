// TODO in descending order of priority
// Delete
// add replay button to replay mid playback
// Progress bar for currently playing audio
// Progress bar for total progress
// Timeline based on clip length
// audio clip waveform (not animated)
// logo shimmer
// fix window resize message not being displayed on app launch
// write tests
// check for preexisting files when doing file check (resume progress)
// delete undo

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
// use std::path::Path;

// rodio is our audio player
// use rodio::Sink;

//public debug
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub mod audio_functions;
pub mod control_functions;
pub mod file_functions;
pub mod helper_functions;
pub mod sorting_functions;
pub mod terminal_functions;

use control_functions::eval_keypress::eval_keypress;
use file_functions::copy_audio::copy_audio;
use file_functions::get_file_list::get_file_list;
use sorting_functions::sort_numbered_file::sort_numbered_files;

use crate::audio_functions::audio_controls::{
    speed_down, speed_reset, speed_up, volume_down, volume_up,
};
use crate::audio_functions::create_sink::{create_sink, PackagedSink};
//use crate::control_functions::eval_keypress::eval_keypress;
use crate::file_functions::get_dir::get_dir;
//use crate::audio_functions::play_audio_file::play_audio_file;
use crate::helper_functions::graceful_shutdown::graceful_shutdown;
use crate::terminal_functions::set_size::set_size;
use crate::terminal_functions::terminal_setup::terminal_setup;

fn main() {
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
    let mut packed: PackagedSink = match create_sink() {
        Ok(ok) => ok,
        Err(err) => graceful_shutdown(
            format!("[main] : Error creating the audio sink and stream: {err:#?}").as_str(),
            1,
        ),
    };

    // now we need to get our directory and copy the audio files to a new folder
    let master_dir: String = get_dir();
    let child_dir: String = copy_audio(master_dir);

    // then make a list of all audio files
    let mut file_list: Vec<String> = get_file_list(child_dir);

    // now we need to sort them

    file_list = sort_numbered_files(&file_list);

    // need an index for the list
    let mut list_index:usize = 0;

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
        // listen and act on keypresses
        (packed, file_list, list_index) = eval_keypress(packed, file_list, list_index);
    }
}
