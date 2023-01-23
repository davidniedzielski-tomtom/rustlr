#[derive(Clone)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

#[derive(Clone)]
pub struct LogEntry {
    level: LogLevel,
    txt: String,
}

impl LogEntry {
    pub fn new(level: LogLevel, txt: String) -> LogEntry {
        LogEntry { level, txt }
    }
}

impl From<u8> for LogLevel {
    fn from(item: u8) -> Self {
        match item {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warn,
            4 => LogLevel::Error,
            _ => LogLevel::Fatal,
        }
    }
}
