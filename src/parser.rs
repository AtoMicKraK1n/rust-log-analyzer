use crate::models::{LogEntry, LogLevel};

pub fn parse_log_line(line: &str, line_number: usize) -> LogEntry {
    let trimmed = line.trim();

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

pub fn extract_location(line: &str) -> (Option<String>, Option<u32>) {
    if let Some(pos) = line.find(".rs:") {
        let before = &line[..pos + 3];
        
        let file_start = before.rfind(|c: char| c == ' ' || c == '\'')
            .map(|i| i + 1)
            .unwrap_or(0);
        
        let file = before[file_start..].to_string();

        let after = &line[pos + 4..];
        if let Some(colon_pos) = after.find(':') {
            if let Ok(line_num) = after[..colon_pos].parse::<u32>() {
                return (Some(file), Some(line_num));
            }
        }
        
        return (Some(file), None);
    }
    
    (None, None)
}