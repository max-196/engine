use log::{LevelFilter, SetLoggerError};

static mut LOGGER: Logger = Logger { max_level: LevelFilter::Off };

pub struct Logger {
    max_level: LevelFilter,
}

impl Logger {
    pub unsafe fn init(max_level: LevelFilter) -> Result<(), SetLoggerError> {
        LOGGER.max_level = max_level;
        log::set_max_level(max_level);
        log::set_logger(&LOGGER)
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) { }
}