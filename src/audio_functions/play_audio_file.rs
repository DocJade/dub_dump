// use rodio::{Decoder, Sink};
use crate::debug_println;
use std::{fs::File, path::Path};

use super::create_sink::PackagedSink;

#[derive(Debug)]
pub enum FilePlayError {
    CannotFindFile,
    Unknown(String),
}

/// Plays an audio file at the specified path.
/// Gives back the Sink when done.
///
/// # Errors
///
/// * `FilePlayError::Unknown`: New error that is not in the `FilePlayError` enum yet.
///
/// # Panics
/// will panic if we see a new kind of file error
pub fn play_audio_file(file_path: &Path, packed: &mut PackagedSink) -> Result<(), FilePlayError> {
    debug_println!(
        "[play_audio_file] : Attempting to play: {:?}...",
        file_path.to_str()
    );

    // Open the file
    let sound: File = match File::open(file_path) {
        Ok(ok) => ok,
        Err(err) => match err {
            _ if err
                .to_string()
                .contains("The system cannot find the file specified. (os error 2)") =>
            {
                return Err(FilePlayError::CannotFindFile)
            }
            _ => panic!("Uncaught file related error!"),
        },
    };

    debug_println!("[play_audio_file] : File opened...");

    // Decode the sound
    let decoder = match rodio::Decoder::new(sound) {
        Ok(ok) => ok,
        Err(err) => return Err(FilePlayError::Unknown(err.to_string())),
    };

    debug_println!("[play_audio_file] : Sound decoded...");

    //play
    packed.sink.append(decoder);

    debug_println!("[play_audio_file] : Sound playing!");

    Ok(())
}