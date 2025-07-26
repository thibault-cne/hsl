use core::fmt;

pub static mut LOGGER: Logger = Logger::new(LogLevel::Info);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => f.write_str("INFO"),
            Self::Warning => f.write_str("WARNING"),
            Self::Error => f.write_str("ERROR"),
        }
    }
}

pub struct Logger {
    threshold: LogLevel,
}

impl Logger {
    pub const fn new(threshold: LogLevel) -> Self {
        Self { threshold }
    }

    pub fn set_threshold(&mut self, new_threshold: LogLevel) {
        self.threshold = new_threshold;
    }

    pub fn log(&self, level: LogLevel, args: fmt::Arguments<'_>) {
        if self.threshold > level {
            return;
        }

        eprintln!("[{level}] {}", args);
    }
}

macro_rules! log {
    ($level:ident, $($args:tt)*) => {
        // SAFETY: this is safe cause we know we're not mutating LOGGER
        // anywhere else than once in main
        unsafe {
            $crate::log::LOGGER.log($crate::log::LogLevel::$level, format_args!($($args)*))
        }
    };
}

macro_rules! info {
    ($($args:tt)*) => {
        log!(Info, $($args)*)
    };
}

macro_rules! warning {
    ($($args:tt)*) => {
        log!(Warning, $($args)*)
    };
}

macro_rules! error {
    ($($args:tt)*) => {
        log!(Error, $($args)*)
    };
}
