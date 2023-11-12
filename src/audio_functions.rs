use rodio::{Decoder, Sink};
use std::{fs::File, path::Path};
use crate::debug_println;

#[derive(Debug)]
pub enum FilePlayError {
    Unknown(String)
}

pub fn play_audio_file(file_path: &Path, sink: &Sink) -> Result<(), FilePlayError> {
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