#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Unknown,
}

#[derive(Debug)]
pub enum RustErrorType {
    Panic,
    ResultError,
    Unwrap,
    Expect,
}

#[derive(Debug)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub line_number: usize,
}

#[derive(Debug)]
pub struct RustError {
    pub error_type: RustErrorType,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub log_line: usize,
}