use crate::models::{RustError, RustErrorType};
use crate::parser::extract_location;

pub fn detect_rust_panic(line: &str, line_number: usize) -> Option<RustError>{
    if line.contains("panicked at") {
        let message = if let Some(start) = line.find("panicked at '") {
            let msg_start = start + "panicked at '".len();
            if let Some(end) = line[msg_start..].find("'") {
                line[msg_start..msg_start + end].to_string()
            } else {
                line[msg_start..].to_string()
            }
        } else {
            "Unknown panic".to_string()
        };

        let (file, line_num) = extract_location(line);

        return Some(RustError {
            error_type: RustErrorType::Panic, 
            message, 
            file, 
            line: line_num, 
            log_line: line_number
        });
    }
    None
}

pub fn detect_rust_error(line: &str, line_number: usize) -> Option<RustError> {
    // Check for unwrap failures
    if line.contains("called `Option::unwrap()` on a `None` value") 
        || line.contains("called `unwrap()` on a `None` value") {
        return Some(RustError {
            error_type: RustErrorType::Unwrap,
            message: "Unwrap called on None".to_string(),
            file: None,
            line: None,
            log_line: line_number,
        });
    }

    if line.contains("called `Result::expect()` on an `Err` value")
        || line.contains("called `expect()` on an `Err` value") {
        let message = if let Some(start) = line.find("expect()` on an `Err` value: ") {
            let msg_start = start + "expect()` on an `Err` value: ".len();
            line[msg_start..].to_string()
        } else {
            "Expect called on Err".to_string()
        };

        return Some(RustError {
            error_type: RustErrorType::Expect,
            message,
            file: None,
            line: None,
            log_line: line_number,
        });
    }

    if line.trim().starts_with("Error: ") {
        let message = line.trim()["Error: ".len()..].to_string();
        
        return Some(RustError {
            error_type: RustErrorType::ResultError,
            message,
            file: None,
            line: None,
            log_line: line_number,
        });
    }

    None
}