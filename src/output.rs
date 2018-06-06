/// A 'raw' error, with no file / lineno information.
macro_rules! error_raw {
        () => (print!("\n"));
        ($fmt:expr) => (print!(concat!("Error: ", $fmt, "\n")));
        ($fmt:expr, $($arg:tt)*) => (print!(concat!("\x1b[31mError: \x1b[0m", $fmt, "\n"), $($arg)*));
}
