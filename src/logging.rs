use log::{Level, Log};

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            serial0_print!("[");
            match record.level() {
                Level::Error => serial0_print!("\x1b[1;31mERROR\x1b[0m"),
                Level::Warn => serial0_print!("\x1b[1;33mWARN\x1b[0m"),
                Level::Info => serial0_print!("\x1b[1;34mINFO\x1b[0m"),
                Level::Debug => serial0_print!("\x1b[1;32mDEBUG\x1b[0m"),
                Level::Trace => serial0_print!("\x1b[1;37mTRACE\x1b[0m"),
            }
            serial0_print!("] {}: {}\n", record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(Level::Trace.to_level_filter());
}
