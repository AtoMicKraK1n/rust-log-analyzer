use crate::models::{LogEntry, LogLevel, RustError, RustErrorType};

pub fn display_rust_errors(rust_errors: &[RustError]) {
    if rust_errors.is_empty() {
        println!("=== Rust Errors ===");
        println!("No Rust-specific errors detected");
        println!();
        return;
    }

    println!("=== Rust Errors Detected ===");
    println!("Found {} Rust error(s)\n", rust_errors.len());

    for (i, error) in rust_errors.iter().enumerate() {
        let error_type_str = match error.error_type {
            RustErrorType::Panic => "PANIC",
            RustErrorType::ResultError => "RESULT ERROR",
            RustErrorType::Unwrap => "UNWRAP FAILURE",
            RustErrorType::Expect => "EXPECT FAILURE",
        };

        println!("{}. [{}] at log line {}", i + 1, error_type_str, error.log_line);
        println!("   Message: {}", error.message);
        
        if let Some(ref file) = error.file {
            if let Some(line) = error.line {
                println!("   Location: {}:{}", file, line);
            } else {
                println!("   File: {}", file);
            }
        }
        println!();
    }
}

pub fn display_summary(entries: &[LogEntry]) {
    let mut error_count = 0;
    let mut warning_count = 0;
    let mut info_count = 0;
    let mut debug_count = 0;
    let mut unknown_count = 0;

    for entry in entries {
        match entry.level {
            LogLevel::Error => error_count += 1,
            LogLevel::Warning => warning_count += 1,
            LogLevel::Info => info_count += 1,
            LogLevel::Debug => debug_count += 1,
            LogLevel::Unknown => unknown_count += 1,
        }
    }

    println!(" Log Analysis Summary ");
    println!("Total entries: {}", entries.len());
    println!("Errors:   {}", error_count);
    println!("Warnings: {}", warning_count);
    println!("Info:     {}", info_count);
    println!("Debug:    {}", debug_count);
    println!("Unknown:  {}", unknown_count);
    println!();
}

pub fn display_entries(entries: &[LogEntry]) {
    println!(" Log Entries ");
    
    for entry in entries {
        let prefix = match entry.level {
            LogLevel::Error => "[ERROR]",
            LogLevel::Warning => "[WARN ]",
            LogLevel::Info => "[INFO ]",
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Unknown => "[?????]",
        };
        
        println!("{} Line {}: {}", prefix, entry.line_number, entry.message);
    }
}


