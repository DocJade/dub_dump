/// This function will print a message, then exit the process.
pub fn graceful_shutdown(message: &str, code: i32) -> ! {
    debug_println!("[graceful_shutdown] : Shutting down!");

    println!("Shutting down: {message}");
    std::process::exit(code);
}
