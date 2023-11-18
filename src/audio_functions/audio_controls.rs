// this file handles keyboard inputs related to audio playback.

use std::ops::Index;

use crate::{helper_functions::graceful_shutdown::graceful_shutdown, Statistics};

use super::{create_sink::PackagedSink, play_audio_file::play_audio_file};

#[derive(Debug)]
pub enum AudioControlError {
    Unknown(String),
}

pub fn play(packed: &PackagedSink) {
    // resume playback
    packed.sink.play();
}

pub fn pause(packed: &PackagedSink) {
    // pause playback
    packed.sink.pause();
}

pub fn toggle_play(packed: &PackagedSink) {
    // if paused, play, else pause
    if packed.sink.is_paused() {
        play(packed);
    } else {
        pause(packed);
    }
}

pub fn best_space(packed: PackagedSink, current_file: &str) -> PackagedSink {
    // test if the sound buffer is empty
    if packed.sink.empty() {
        debug_log!("No file playing, replaying...");
        // out of sounds! replay the current file
        match play_audio_file(std::path::Path::new(&current_file), &packed) {
            Ok(()) => {}
            Err(err) => graceful_shutdown(
                format!("[best_space] : could not replay file! {err:?}").as_str(),
                1,
            ),
        };
    } else {
        // buffer is not empty
        debug_log!("Buffer is not empty, toggling...");
        toggle_play(&packed);
    }
    packed
}

pub fn stop(packed: &PackagedSink) {
    // stop playback
    packed.sink.stop();
}

pub fn volume_up(packed: &PackagedSink) {
    // increase volume
    packed.sink.set_volume(packed.sink.volume() + 0.1);
}

pub fn volume_down(packed: &PackagedSink) {
    // decrease volume
    packed.sink.set_volume(packed.sink.volume() - 0.1);
}

pub fn get_volume(packed: &PackagedSink) -> f32 {
    debug_log!("volume is {}.", packed.sink.volume());
    packed.sink.volume()
}

pub fn speed_up(packed: &PackagedSink) {
    // speed up
    packed.sink.set_speed(packed.sink.speed() + 0.1);
}

pub fn speed_down(packed: &PackagedSink) {
    // slow down
    packed.sink.set_speed(packed.sink.speed() - 0.1);
}

pub fn speed_reset(packed: &PackagedSink) {
    // 100% speed
    packed.sink.set_speed(1.0);
    debug_log!("Speed reset to 100%");
}

#[must_use]
pub fn skip(
    packed: PackagedSink,
    file_list: Vec<String>,
    index: usize,
    stats: Statistics
) -> (PackagedSink, Vec<String>, usize, Statistics) {
    // make sure we can actually skip

    if index + 1 > file_list.len() {
        // too far!
        debug_log!("Tried to skip, but hit end of vec.");
        return (packed, file_list, index, stats);
    }

    // we're good!

    let new_index = index + 1;

    // stop previous audio

    debug_log!("Stopping previous sound...");
    stop(&packed);

    // play new file.

    debug_log!("Playing new sound...");
    match play_audio_file(
        std::path::Path::new(file_list.index(new_index)),
        &packed,
    ) {
        Ok(()) => {}
        Err(err) => graceful_shutdown(format!("[skip] : could not play file! {err:?}").as_str(), 1),
    }
    // file is playing! return values.
    (packed, file_list, new_index, stats)
}

#[must_use]
pub fn skip_back(
    packed: PackagedSink,
    file_list: Vec<String>,
    index: usize,
    stats: Statistics,
) -> (PackagedSink, Vec<String>, usize, Statistics) {
    // make sure we can actually skip

    if index == 0 {
        // too far!
        debug_log!("Tried to skip, but hit beginning of vec.");
        
        return (packed, file_list, index, stats);
    }

    // we're good!

    let new_index = index - 1;

    // stop previous audio

    debug_log!("[skip] : Stopping previous sound...");
    stop(&packed);

    // play new file.

    debug_log!("Playing new sound...");
    match play_audio_file(
        std::path::Path::new(file_list.index(new_index)),
        &packed,
    ) {
        Ok(()) => {}
        Err(err) => graceful_shutdown(format!("[skip] : could not play file! {err:?}").as_str(), 1),
    }
    // file is playing! return values.
    (packed, file_list, new_index, stats)
}
