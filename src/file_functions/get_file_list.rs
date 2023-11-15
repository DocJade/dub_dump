// return a vec of strings containing each file
use crate::graceful_shutdown;
use std::fs::{self, DirEntry};

#[must_use]
pub fn get_file_list(copy_path: String) -> Vec<String> {
    // create output vec

    let mut output: Vec<String> = vec![];
    let paths = match fs::read_dir(copy_path) {
        Ok(ok) => ok,
        Err(err) => graceful_shutdown(
            format!("[get_file_list] : Unable unwrap path! : {err}").as_str(),
            1,
        ),
    };

    let mut unwrapped_path: DirEntry;

    for path in paths {
        match path {
            Ok(ok) => unwrapped_path = ok,
            Err(err) => graceful_shutdown(format!("[get_file_list] : !? : {err}").as_str(), 1),
        };

        unwrapped_path.path().to_str().map_or_else(
            || graceful_shutdown("[get_file_list] : Unable to cast path to String!", 1),
            |some| output.push(some.to_string()),
        );
    }
    output
}
