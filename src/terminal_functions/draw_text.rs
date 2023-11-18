use colored::Colorize;
use crossterm::{cursor::MoveTo, ExecutableCommand};
use std::io::{self};

pub struct PrintableText {
    pub text_color: colored::Color,
    pub message: String,
    pub pos_x: u16,
    pub pos_y: u16,
}
#[derive(Debug)]
pub enum DrawError {
    Unknown(String),
}

/// This function takes in text, a color, and its position
/// then plops it onto the screen.
///
/// # Errors
/// `DrawError::Unknown` uncaught error basically my brudda
pub fn draw_text(text: &PrintableText) -> Result<(), DrawError> {
    let final_text: String = format!("{}", text.message.color(text.text_color)); // color the text
    match io::stdout().execute(MoveTo(text.pos_x, text.pos_y)) {
        Ok(_) => {}
        Err(err) => return Err(DrawError::Unknown(err.to_string())),
    }; // move the cursor
    print!("{final_text}"); // print!
    Ok(())
}

// now make a macro for easier drawing, if it fails, exit program.
#[allow(clippy::module_name_repetitions)] // yes clippy i know, this is the _easy_ variant and i would like to name it as such
pub fn easy_draw_text(string: String, pos_x: u16, pos_y: u16) {
    match draw_text(&PrintableText {
        text_color: colored::Color::White,
        message: string,
        pos_x,
        pos_y,
    }) {
        Ok(_) => {}
        Err(err) => crate::helper_functions::graceful_shutdown::graceful_shutdown(
            &format!("[MACRO][easy_draw_text] : Issue drawing {err:#?}"),
            1,
        ),
    }
}
