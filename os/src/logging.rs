use core::arch::asm;
use log::{Level, Metadata, Record};
struct SimpleLogger;
use log::{LevelFilter, SetLoggerError};
use owo_colors::AnsiColors::{Blue, Cyan, Green, Red, Yellow};

use owo_colors::OwoColorize;

use crate::timer::timer_now;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}

fn cpuid() -> usize {
    let mut cpuid;
    unsafe {
        asm!("mv {cpuid}, tp",
            cpuid = out(reg) cpuid)
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
                Level::Error => "[ERRO]".color(Red),
                Level::Warn => "[WARN]".color(Yellow),
                Level::Info => "[INFO]".color(Blue),
                Level::Debug => "[DEBU]".color(Green),
                Level::Trace => "[TRAC]".color(Cyan),
            };

            let file = record.file().map_or("", |s| s);
            let line = record.line().map_or(0, |s| s);

            println!(
                "{}{:.6}{}{}{}{}{} {} {}:{}  {}",
                "[".green(),
                timer_now().as_secs_f64().green(),
                "]".green(),
                "[K]".green(),
                "[".green(),
                cpuid().green(),
                "]".green(),
                level.bold(),
                file.cyan(),
                line.cyan(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
