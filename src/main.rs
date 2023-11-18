// TODO in descending order of priority
// Progress bar for currently playing audio
// Progress bar for total progress
// audio clip waveform (not animated)
// statistics window
// Timeline based on clip length
// logo shimmer
// write tests
// check for preexisting files when doing file check (resume progress)
// delete undo
// track audio length inside of file list to never recalculate the file lengths more than once.

// figure out why i cannot print to col 80

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
macro_rules! debug_log {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::log::debug!($($arg)*); })
}

pub mod audio_functions;
pub mod control_functions;
pub mod file_functions;
pub mod helper_functions;
pub mod sorting_functions;
pub mod terminal_functions;

use crate::helper_functions::get_runtime::get_runtime;
use crate::terminal_functions::draw_non_static::draw_non_static;
use crate::update_statistics::update_statistics;

use control_functions::eval_keypress::eval_keypress;
use file_functions::copy_audio::copy_audio;
use file_functions::get_file_list::get_file_list;
use helper_functions::setup_logging::setup_logging;
use sorting_functions::sort_numbered_file::sort_numbered_files;
use terminal_functions::title_screen::draw_static_bits;

use crate::audio_functions::audio_controls::{
    speed_down, speed_reset, speed_up, volume_down, volume_up,
};
use crate::audio_functions::create_sink::{create_sink, PackagedSink};
//use crate::control_functions::eval_keypress::eval_keypress;
use crate::file_functions::get_dir::get_dir;
//use crate::audio_functions::play_audio_file::play_audio_file;
use crate::helper_functions::graceful_shutdown::graceful_shutdown;
use crate::helper_functions::pick_splash::pick_splash;
use crate::helper_functions::update_statistics;
use crate::terminal_functions::set_size::set_size;
use crate::terminal_functions::terminal_setup::terminal_setup;

//Constants

const VERSION_STRING: &str = "Version 0.0.0 Nov 16 2022";

// struct for statistics

// all time stats are in seconds
#[derive(Debug)]
pub struct Statistics {
    total_clips: usize,  // total clips ever imported this session
    dumped_clips: usize, // total deleted clips
    cut_ratio: f64, // ratio of kept to thrown away, throwing away none is 0, all is 1 (cut/total)
    old_run_time: f64, // original runtime of all the files
    new_run_time: f64, // current run time of all files
    time_saved: f64, // old - new
}

fn main() {
    // start logger if we are in a debug build
    if ::std::cfg!(debug_assertions) {
        setup_logging();
        debug_log!("Hello world!");
    }
    //bring the splashes with us
    let splashes: &[u8] = include_bytes!("splashes.txt");

    // get a random splash

    debug_log!("Picking splash...");
    let splash_text = pick_splash(splashes);

    debug_log!("Splash picked. \"{splash_text}\"");
    // do terminal setup

    match terminal_setup() {
        Ok(_) => {
            debug_log!("Terminal ready!");
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
    debug_log!("Asking user for directory...");
    let master_dir: String = get_dir();
    debug_log!("Cloning directory...");
    println!("Cloning audio...");
    let child_dir: String = copy_audio(master_dir);
    
    debug_log!("Listing files...");
    println!("Listing files...");
    // then make a list of all audio files
    let mut file_list: Vec<String> = get_file_list(child_dir);
    
    // now we need to sort them
    
    debug_log!("Sorting files...");
    println!("Sorting files...");
    file_list = sort_numbered_files(&file_list);

    // need an index for the list
    let mut list_index: usize = 0;

    // begin tracking statistics
    let mut statistics = Statistics {
        total_clips: 0,
        dumped_clips: 0,
        cut_ratio: 0.0,
        old_run_time: 0.0,
        new_run_time: 0.0,
        time_saved: 0.0,
    };

    //update
    
    debug_log!("Calculating initial statistics...");
    println!("Calculating audio length, this may take a while...");
    statistics = update_statistics(statistics, file_list.len(), 0, get_runtime(file_list.clone()), 0.0);
    debug_log!("Done!");

    // check terminal size

    match set_size() {
        Ok(_) => {}
        Err(err) => graceful_shutdown(
            format!("[main] : error with terminal sizing: {err:#?}").as_str(),
            1,
        ),
    };

    // print the main screen
    debug_log!("Drawing static elements...");
    draw_static_bits(splash_text, VERSION_STRING.to_string());
    
    debug_log!("Drawing non-static elements...");
    draw_non_static(&statistics, list_index);

    // now we shall enter the main loop
    debug_log!("Entering main loop!");
    loop {
        // listen and act on keypresses
        (packed, file_list, list_index) = eval_keypress(packed, file_list, list_index);
        draw_non_static(&statistics, list_index);
    }
}

// tests
#[test]
fn have_splashes_test() {
    // check to make sure splashes.txt isnt empty
    let splashes = include_bytes!("splashes.txt");
    assert!(!splashes.is_empty()); // make sure it isnt empty
}

#[test]
fn properly_sized_splashes() {
    // make sure no splashes are more than 56 char wide.
    let splashes = include_bytes!("splashes.txt");
    let split_splashes: Vec<&str> = match std::str::from_utf8(splashes) {
        Ok(text) => text.split('\n').collect(),
        Err(err) => panic!("Unable to split splashes.txt! : {err}"),
    };
    for splash in split_splashes {
        assert!(splash.len() <= 56);
    }
}

#[test]
fn no_empty_splashes() {
    // make sure no splashes are empty.
    let splashes = include_bytes!("splashes.txt");
    let split_splashes: Vec<&str> = match std::str::from_utf8(splashes) {
        Ok(text) => text.split('\n').collect(),
        Err(err) => panic!("Unable to split splashes.txt! : {err}"),
    };
    for splash in split_splashes {
        assert!(!splash.is_empty());
    }
}

#[test]
fn no_newline_at_end_of_splashes() {
    // make sure there isnt a newline at the end of the splash file
    let splashes = include_bytes!("splashes.txt");
    assert_ne!(splashes.last(), Some(&b'\n')); // make sure it isnt empty
}
