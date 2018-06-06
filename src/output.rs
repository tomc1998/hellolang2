/// A 'raw' error, with no file / lineno information.
macro_rules! error_raw {
    () => (println!());
    ($fmt:expr) => (println!(concat!("Error: ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("\x1b[31mError:\x1b[0m ", $fmt), $($arg)*));
}

macro_rules! error_point {
    ($msg:expr, $file:expr, $point:expr) =>
        (println!("\x1b[31mError:\x1b[0m {}:{} - {}", $file, $point, $msg))
}
