//todo
// prompt the user for a directory

use crate::graceful_shutdown;
use std::path::Path;

use crossterm::terminal;

// string is checked for validity before returning.

#[must_use] // gotta use that string m8
pub fn get_dir() -> String {
    // prompt time!
    // loop will break on proper input
    loop {
        //get user input
        use std::io::{stdin, stdout, Write};
        let mut s = String::new();
        println!("Please enter audio directory: ");
        // we cannot readline in raw mode, need to switch.

        match terminal::disable_raw_mode() {
            Ok(()) => {}
            Err(err) => graceful_shutdown(&format!("[get_dir] Unable to disable raw mode! {err}"), 1),
        };

        let _ = stdout().flush();
        match stdin().read_line(&mut s) {
            Ok(_) => {}
            Err(_) => {
                continue; /* something broke, try again */
            }
        }
        if s.ends_with('\n') {
            s.pop();
        }
        if s.ends_with('\r') {
            s.pop();
        }

        if Path::new(&s).is_dir() {
            // thats a directory!
            // we don't care if theres actually audio in there,
            // why would someone enter a dir with no audio?

            //re-enable raw mode
            match terminal::enable_raw_mode() {
                Ok(()) => {}
                Err(err) => graceful_shutdown(&format!("[get_dir] Unable to re-enable raw mode! {err}"), 1),
            };

            return s;
        }
        println!("That is not a directory, try again.");
    }
}
