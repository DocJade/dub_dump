use std::fs::File;

use rodio::Source;

use crate::helper_functions::graceful_shutdown::graceful_shutdown;

// TODO
// rewrite this, why cannot i get the file length directly?

// take in a list of files and calculate the total runtime in seconds in F64
pub fn get_runtime(files: Vec<String>) -> f64 {
    debug_log!("Calculating runtime...");
    let mut total: f64 = 0.0;
    for file in files {
        // Open the file
        let sound1: File = match File::open(file.clone()) {
            Ok(ok) => ok,
            Err(err) => graceful_shutdown(
                &format!("[get_runtime] : Couldn't open audio file! {err}"),
                1,
            ),
        };
        let sound2: File = match File::open(file.clone()) {
            Ok(ok) => ok,
            Err(err) => graceful_shutdown(
                &format!("[get_runtime] : Couldn't open audio file! {err}"),
                1,
            ),
        };
        // Decode the sound
        // TODO benchmark this, is decoding each file slow? can we do it faster?
        let decoder1 = match rodio::Decoder::new(sound1) {
            Ok(ok) => ok,
            Err(err) => graceful_shutdown(
                &format!("[get_runtime] : Couldn't decode audio file! {err}"),
                1,
            ),
        };
        let decoder2 = match rodio::Decoder::new(sound2) {
            Ok(ok) => ok,
            Err(err) => graceful_shutdown(
                &format!("[get_runtime] : Couldn't decode audio file! {err}"),
                1,
            ),
        };
        // calculate sound length in seconds

        let samples = decoder1.count();
        let sample_rate = decoder2.sample_rate();
        let length = (samples as f64 / sample_rate as f64);
        // debug_log!("File:{}, Length:{}s", file, length);
        // get length
        total += length;
    }

    // counted all files!
    return total; //temp
}
