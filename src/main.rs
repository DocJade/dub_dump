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
use std::{fs::File, path::Path};

// rodio is our audio player
use rodio::{Decoder, OutputStream, Sink};

// tui-rs is our terminal interface
// use tui::*;

//debug only printing
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

fn main() {
    // create our audio sink

    // First the stream
    let (_stream, stream_handle) = match OutputStream::try_default(){
        Ok(ok) => ok, // sweet!
        Err(err) => graceful_shutdown(format!("error creating stream: {err:#?}").as_str(), 1), // uh oh
    };

    debug_println!("Stream created...");

    // Then let that sink in
    let sink: Sink = match Sink::try_new(&stream_handle){
        Ok(ok) => ok, // sweet!
        Err(err) => graceful_shutdown(format!("error creating sink: {err:#?}").as_str(), 1), // uh oh
    };

    debug_println!("Sink done!");
    
    // play a sound
    let play_result = play_audio_file(Path::new("test.wav"), &sink);
    match play_result {
        Ok(_) => {}, // swag, move on.
        Err(err) => graceful_shutdown(format!("error playing sound: {err:#?}").as_str(), 1)
    }

    // close program when sound is done

    debug_println!("waiting for sound to finish playing...");
    sink.sleep_until_end();
    debug_println!("sound has finished playing.");
    sink.stop();
    graceful_shutdown("done!", 0)
}

fn graceful_shutdown(message: &str, code: i32) -> ! {
    println!("Shutting down: {message}");
    std::process::exit(code);
}

#[derive(Debug)]
enum FilePlayError {
    Unknown(String)
}

fn play_audio_file(file_path: &Path, sink: &Sink) -> Result<(), FilePlayError> {
    debug_println!("attempting to play: {:#?}...", file_path.to_str());
    
    // Open the file
    let sound:File = match File::open(file_path) {
        Ok(ok) => ok,
        Err(err) => return Err(FilePlayError::Unknown(err.to_string())),
    };

    debug_println!("File opened...");
    
    // Decode the sound
    let decoder = match Decoder::new(sound){
        Ok(ok) => ok,
        Err(err) => return Err(FilePlayError::Unknown(err.to_string())),
    };

    debug_println!("Sound decoded...");

    //play
    sink.append(decoder);

    debug_println!("Sound playing!");

    Ok(())
}