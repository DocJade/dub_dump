use log4rs::config::Deserializers;

/// # Panics
///
/// Will panic if the logger is unable to start, but this only runs on debug builds.
pub fn setup_logging() {
    match log4rs::init_file("log4rs.yml", Deserializers::default()) {
        Ok(()) => {}
        Err(err) => panic!("Unable to start logger! : {err}"),
    };
}

#[test]
fn test_log_config() {
    match log4rs::init_file("log4rs.yml", Deserializers::default()) {
        Ok(()) => {}
        Err(err) => panic!("Unable to start logger! : {err}"),
    };
}
