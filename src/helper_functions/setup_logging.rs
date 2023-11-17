use log4rs::config::Deserializers;

pub fn setup_logging() {
    match log4rs::init_file("log4rs.yml", Deserializers::default()){
        Ok(_) => {},
        Err(err) => panic!("Unable to start logger! : {err}"),
    };
}