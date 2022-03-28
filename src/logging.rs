use core::arch::asm;
use log::{Level, Metadata, Record};
struct SimpleLogger;

use log::{LevelFilter, SetLoggerError};
use owo_colors::AnsiColors::{Blue, Cyan, Green, Red, Yellow};
use owo_colors::OwoColorize;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}

fn cpuid() -> usize {
    let mut cpuid = 0usize;
    unsafe {
        asm!(
            "mv {cpuid}, tp",
            cpuid = inout(reg) cpuid,
        );
    }
    cpuid
}

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                Level::Error => "ERRO".color(Red),
                Level::Warn => "WARN".color(Yellow),
                Level::Info => "INFO".color(Blue),
                Level::Debug => "DEBU".color(Green),
                Level::Trace => "TRAC".color(Cyan),
            };

            println!("[{}] [{}] {}", level, cpuid(), record.args());
        }
    }

    fn flush(&self) {}
}
