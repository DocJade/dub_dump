#[derive(PartialEq, Clone)]
struct TSize {
    x: u16,
    y: u16,
}
use crate::terminal_functions::draw_text::draw_text;
use crate::terminal_functions::draw_text::PrintableText;
use crossterm::terminal::{self, ClearType};
const DESIRED_SIZE: TSize = TSize { x: 80, y: 30 };

use crossterm::{terminal::Clear, ExecutableCommand};
use std::io::{self};

use super::draw_text::DrawError;

#[derive(Debug)]
pub enum TerminalSizeError {
    DrawError(DrawError),
    Unknown(String),
}

/// Resize the terminal window
/// This function will not exit until the user has resized the terminal
/// to the proper size.
///
/// # Errors
///
/// `TerminalSizeError::Unknown`, undocumented terminal error.
pub fn set_size() -> Result<(), TerminalSizeError> {
    // get the terminal size
    let mut size: TSize;
    let mut last_size: TSize = TSize { x: 0, y: 0 };
    let mut message: String;
    loop {
        size = match terminal::size() {
            Ok(s) => TSize { x: s.0, y: s.1 },
            Err(err) => return Err(TerminalSizeError::Unknown(err.to_string())),
        };
        // check if size has changed
        if last_size == size {
            // no change, keep looping
            continue;
        }

        // new size! update the old one
        last_size = size.clone();

        if size == DESIRED_SIZE {
            // terminal is the correct size, we are done here.
            return Ok(());
        }
        // not the right size

        // clear terminal and print resize message.
        match io::stdout().execute(Clear(ClearType::All)) {
            Ok(_) => {}
            Err(err) => return Err(TerminalSizeError::Unknown(err.to_string())),
        };
        // generate string to print
        message = format!(
            "Please resize terminal to be {} by {}.\nCurrent size is {} by {}.\n",
            DESIRED_SIZE.x, DESIRED_SIZE.y, size.x, size.y
        );
        // throw the message in the top left.
        let printer: PrintableText = PrintableText {
            text_color: colored::Color::Red,
            message,
            pos_x: 0,
            pos_y: 0,
        };
        // finally, print it.
        match draw_text(&printer) {
            Ok(_) => {}
            Err(err) => return Err(TerminalSizeError::DrawError(err)),
        };
    }
}
