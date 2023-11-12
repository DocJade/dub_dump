// TODO
// Playing audio files
// Progress bar for total progress
// Progress bar for currently playing audio
// Timeline based on clip length
// Volume control

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
use std::{fs::File, thread::*, time::Duration};

// rodio is our audio player
use rodio::{Decoder, OutputStream, Sink};

// tui-rs is our terminal interface
// use tui::*;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let decoder = Decoder::new(File::open("test.wav").unwrap()).unwrap();
    sink.append(decoder);

    // Interrupt playback after 5 seconds
    sleep(Duration::from_secs(10));
    sink.stop();
}
