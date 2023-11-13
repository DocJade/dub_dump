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
