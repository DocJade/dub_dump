// this file deals with enabling terminal features we need, then disabling them when we are done.

use crossterm::terminal;
use crossterm::{cursor::DisableBlinking, cursor::Hide, terminal::Clear, ExecutableCommand};
use std::io::{self};

#[derive(Debug)]
pub enum TerminalInitialSetupError {
    Unknown(String),
}

/// Set up the terminal
///
/// # Errors
///
/// `TerminalSetupError::Unknown`, undocumented terminal error.
pub fn terminal_setup() -> Result<(), TerminalInitialSetupError> {
    //disable cursor blinking
    match io::stdout().execute(DisableBlinking) {
        Ok(_) => {}
        Err(err) => return Err(TerminalInitialSetupError::Unknown(err.to_string())),
    };
    //hide cursor
    match io::stdout().execute(Hide) {
        Ok(_) => {}
        Err(err) => return Err(TerminalInitialSetupError::Unknown(err.to_string())),
    };
    //capture all terminal input. //! this disables ^c
    match terminal::enable_raw_mode() {
        Ok(()) => {}
        Err(err) => return Err(TerminalInitialSetupError::Unknown(err.to_string())),
    };

    // enter the alternate terminal
    match io::stdout().execute(terminal::EnterAlternateScreen) {
        Ok(_) => {}
        Err(err) => return Err(TerminalInitialSetupError::Unknown(err.to_string())),
    };

    // clear the terminal

    match io::stdout().execute(Clear(crossterm::terminal::ClearType::All)) {
        Ok(_) => {}
        Err(err) => return Err(TerminalInitialSetupError::Unknown(err.to_string())),
    };
    Ok(())
}
