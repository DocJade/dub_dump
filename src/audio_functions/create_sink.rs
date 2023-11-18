use rodio::{OutputStream, Sink};

#[derive(Debug)]
pub enum StreamCreationError {
    Unknown(String),
}

pub struct PackagedSink {
    pub sink: Sink,
    pub stream: OutputStream,
}

/// Creates and returns the sink for playing audio
///
/// # Errors
///
/// * `StreamCreationError::Unknown`: New error that is not in the `StreamCreationError` enum yet.
pub fn create_sink() -> Result<PackagedSink, StreamCreationError> {
    debug_log!("Attempting to create stream...");

    let (stream, stream_handle) = match OutputStream::try_default() {
        Ok(ok) => ok,                                                          // sweet!
        Err(err) => return Err(StreamCreationError::Unknown(err.to_string())), // uh oh
    };

    debug_log!("Stream created!");

    debug_log!("Attempting to create sink...");
    // let that sink in
    let sink: Sink = match Sink::try_new(&stream_handle) {
        Ok(ok) => ok,                                                          // sweet!
        Err(err) => return Err(StreamCreationError::Unknown(err.to_string())), // uh oh
    };

    debug_log!("Sink created!");
    let packed: PackagedSink = PackagedSink { sink, stream };
    debug_log!("Audio startup good!");
    Ok(packed)
}
