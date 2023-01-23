use crate::errors::OpenLrErr;
use crate::log::LogEntry;
use std::time::Duration;

#[derive(Debug)]
pub struct RequestResult<T> {
    pub id: i64,
    pub result: Result<T, OpenLrErr>,
    pub elapsed: Duration,
    pub log: Vec<LogEntry>,
}

impl<T> RequestResult<T> {
    pub fn new(
        id: i64,
        result: Result<T, OpenLrErr>,
        elapsed: Duration,
        log: Vec<LogEntry>,
    ) -> Self {
        RequestResult {
            id: id,
            result: result,
            elapsed: elapsed,
            log: log,
        }
    }
}
