use std::io;
use std::io::Write;
use paris::formatter::colorize_string;

fn stdout(raw: &str) {
    io::stdout().write_all(raw.as_bytes()).unwrap();
}

fn stderr(raw: &str) {
    io::stderr().write_all(raw.as_bytes()).unwrap();
}

pub fn log_cs(symbol: &str, message: &str, error: bool, newline: bool) {
    let timestamp = chrono::Local::now().format("%I:%M:%S %p");
    let raw = format!("{}: {} {}{}", timestamp, symbol, colorize_string(message), if newline { "\n" } else { "" });
    if error {
        stderr(&raw);
    } else {
        stdout(&raw);
    }
}

pub fn log_c(symbol: &str, message: &str, error: bool) {
    log_cs(symbol, message, error, true);
}

pub fn log(symbol: &str, message: &str) {
    log_c(symbol, message, false);
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (log::log("ℹ", &format!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (log::log("⚠", &format!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (log::log_c("✖", &format!($($arg)*), true));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (log::log("⚡", &format!($($arg)*)));
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => (log::log("✔", &format!($($arg)*)));
}

#[macro_export]
macro_rules! cstm {
    ($symbol:expr, $($arg:tt)*) => (log::log($symbol, &format!($($arg)*)));
}