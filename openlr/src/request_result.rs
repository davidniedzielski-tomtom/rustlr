use crate::errors::OpenLrErr;
use crate::log::LogEntry;
use std::time::Duration;

pub struct RequestResult<T> {
    id: i64,
    result: Result<T, OpenLrErr>,
    elapsed: Duration,
    log: Vec<LogEntry>,
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
