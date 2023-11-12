//debug only printing
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

pub fn graceful_shutdown(message: &str, code: i32) -> ! {
    println!("Shutting down: {message}");
    std::process::exit(code);
}