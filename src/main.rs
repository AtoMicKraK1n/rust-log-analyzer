use std::fs;
use std::env;

mod models;
mod parser;
mod rust_error;
mod display;

use parser::parse_log_line;
use rust_error::{detect_rust_panic, detect_rust_error};
use display::{display_summary, display_entries, display_rust_errors};

use crate::models::LogEntry;
use crate::models::RustError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <logfile>", args[0]);
        eprintln!("Example: {} app.log", args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)?;

    let entries: Vec<LogEntry> = contents
        .lines()
        .enumerate()
        .map(|(i, line)| parse_log_line(line, i + 1))
        .collect();

    let rust_errors: Vec<RustError> = contents
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            // Try panic detection first
            if let Some(error) = detect_rust_panic(line, i + 1) {
                return Some(error);
            }
            // Then try other error patterns
            detect_rust_error(line, i + 1)
        })
        .collect();

    display_summary(&entries);
    display_rust_errors(&rust_errors);
    display_entries(&entries);

    Ok(())
}
