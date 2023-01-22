use crate::log::{LogEntry, LogLevel};
use crate::map::Map;
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

pub struct RequestContext<'a, ParamType> {
    pub map: &'a dyn Map,
    pub params: &'a ParamType,
    log: Arc<Mutex<Vec<LogEntry>>>,
    log_level: u8,
}

impl<'a, ParamType> RequestContext<'a, ParamType> {
    pub fn trace<F>(&self, f: F)
    where
        F: Fn() -> String,
    {
        if self.log_level >= LogLevel::Trace as u8 {
            self.log(LogLevel::Trace, f);
        }
    }
    pub fn debug<F>(&self, f: F)
    where
        F: Fn() -> String,
    {
        if self.log_level >= LogLevel::Debug as u8 {
            self.log(LogLevel::Debug, f);
        }
    }
    pub fn info<F>(&self, f: F)
    where
        F: Fn() -> String,
    {
        if self.log_level >= LogLevel::Info as u8 {
            self.log(LogLevel::Info, f);
        }
    }
    pub fn warn<F>(&self, f: F)
    where
        F: Fn() -> String,
    {
        if self.log_level >= LogLevel::Warn as u8 {
            self.log(LogLevel::Warn, f);
        }
    }
    pub fn error<F>(&self, f: F)
    where
        F: Fn() -> String,
    {
        if self.log_level >= LogLevel::Error as u8 {
            self.log(LogLevel::Error, f);
        }
    }
    pub fn fatal<F>(&self, f: F)
    where
        F: Fn() -> String,
    {
        if self.log_level >= LogLevel::Fatal as u8 {
            self.log(LogLevel::Fatal, f);
        }
    }

    fn log<F>(&self, level: LogLevel, f: F)
    where
        F: Fn() -> String,
    {
        Arc::clone(&self.log)
            .lock()
            .unwrap()
            .push(LogEntry::new(SystemTime::now(), level, f()))
    }

    pub fn get_log(&self) -> Vec<LogEntry> {
        Arc::clone(&self.log).lock().unwrap().clone()
    }

    pub fn new(
        map: &'a dyn Map,
        params: &'a ParamType,
        log_level: LogLevel,
    ) -> RequestContext<'a, ParamType> {
        RequestContext {
            map: map,
            log: Arc::new(Mutex::new(Vec::<LogEntry>::new())),
            params: params,
            log_level: log_level as u8,
        }
    }
}
