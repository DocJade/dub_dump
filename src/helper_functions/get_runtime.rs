use std::fs::File;

use rodio::Source;

use crate::helper_functions::graceful_shutdown::graceful_shutdown;

// TODO
// rewrite this, why cannot i get the file length directly?

// take in a list of files and calculate the total runtime in seconds in F64
#[must_use]
pub fn get_runtime(files: Vec<String>) -> f64 {
    debug_log!("Calculating runtime...");

    let mut total: f64 = 0.0;

    // for sake of speed, we are going to assume all files have the same sample rate as the first one

    // get the first file

    let sample_rate_file = files.first().map_or_else(
        || graceful_shutdown("[get_runtime] : Couldn't open first audio file!", 1),
        |file| file,
    );

    // open it

    let sample_rate_sound: File = match File::open(sample_rate_file) {
        Ok(ok) => ok,
        Err(err) => graceful_shutdown(
            &format!("[get_runtime] : Couldn't open audio file! {err}"),
            1,
        ),
    };

    // decode it

    let sample_rate_decoder = match rodio::Decoder::new(sample_rate_sound) {
        Ok(ok) => ok,
        Err(err) => graceful_shutdown(
            &format!("[get_runtime] : Couldn't decode audio file! {err}"),
            1,
        ),
    };

    // get that sweet sweet sample rate

    let sample_rate = sample_rate_decoder.sample_rate();
    debug_log!("Assuming sample rate is {} from now on.", &sample_rate);

    // now start looping!

    for file in files {
        // Open the file
        let sound1: File = match File::open(file.clone()) {
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
        // calculate sound length in seconds

        let samples = decoder1.count();
        let length = samples as f64 / f64::from(sample_rate);
        // debug_log!("File:{}, Length:{}s", file, length);
        // get length
        total += length;
    }

    // counted all files!
    total //temp
}
