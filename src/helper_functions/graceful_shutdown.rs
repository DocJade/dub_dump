/// This function will print a message, then exit the process.
use crossterm::{terminal, ExecutableCommand};
use std::io::{self};
pub fn graceful_shutdown(message: &str, code: i32) -> ! {
    //shutdown cleanup

    match terminal::disable_raw_mode() {
        Ok(_) => {}
        Err(err) => crate::helper_functions::graceful_shutdown::graceful_shutdown(
            format!("[main] : Failed to enable raw terminal mode: {err:#?}").as_str(),
            1,
        ),
    };
    // exit the alternate terminal
    match io::stdout().execute(terminal::LeaveAlternateScreen) {
        Ok(_) => {}
        Err(err) => crate::helper_functions::graceful_shutdown::graceful_shutdown(
            format!("[main] : Failed to enable alt terminal mode: {err:#?}").as_str(),
            1,
        ),
    };

    debug_println!("[graceful_shutdown] : Shutting down!");

    println!("Shutting down: {message}");
    std::process::exit(code);
}
