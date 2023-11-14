// this file handles keyboard inputs related to audio playback.

use super::create_sink::PackagedSink;

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
    debug_println!("[get_volume] : volume is {}.", packed.sink.volume());
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
    debug_println!("[speed_reset] : Speed reset to 100%");
}
