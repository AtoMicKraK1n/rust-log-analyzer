use std::fs;
use std::env;

#[derive(Debug)]
enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Unknown,
}

#[derive(Debug)]
struct  LogEntry {
    level: LogLevel,
    message: String,
    line_number: usize,
}

fn parse_log_line(line: &str, line_number: usize) -> LogEntry {
    // trimming whitespaces
    let trimmed = line.trim();

    // conditions for type of logs
    let level = if trimmed.contains("ERROR") || trimmed.contains("error:") {
        LogLevel::Error
    } else if trimmed.contains("WARN") || trimmed.contains("warning:") {
        LogLevel::Warning
    } else if trimmed.contains("INFO") || trimmed.contains("info:") {
        LogLevel::Info
    } else if trimmed.contains("DEBUG") || trimmed.contains("debug:") {
        LogLevel::Debug
    } else {
        LogLevel::Unknown
    };

    LogEntry {
        level,
        message: trimmed.to_string(),
        line_number,
    }

}

fn display_summary(entries: &[LogEntry]) {
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

fn display_entries(entries: &[LogEntry]) {
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
        .map(|(i, line)| {
            parse_log_line(line, i + 1)
        })
        .collect();

    display_summary(&entries);
    display_entries(&entries);

    Ok(())
}

