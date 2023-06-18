use std::error::Error;
use std::io::{stdout, Write};
use chrono::{DateTime, Local};
use crossterm::QueueableCommand;
use crossterm::style::{Print, PrintStyledContent, Stylize, Color};
use log::{Log, Record, Level, Metadata};
use log::{SetLoggerError, LevelFilter};

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        write_log(record).expect("could not log");
    }

    fn flush(&self) {}
}

fn write_log(record: &Record) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    stdout.queue({
        let ts = Local::now();
        let ts_text = ts.format("%v %r").to_string();
        PrintStyledContent(ts_text.dim())
    })?;

    stdout.queue(Print(" "))?;

    stdout.queue({
        let level = record.level();
        let color = get_level_color(level);
        let text = get_level_text(level);
        PrintStyledContent(text.with(color).bold())
    })?;

    stdout.queue(Print(" "))?;

    stdout.queue({
        Print(record.args().to_string())
    })?;

    stdout.queue(Print("\n"))?;

    stdout.flush()?;

    Ok(())
}

fn get_level_color(level: Level) -> Color {
    match level {
        Level::Error => Color::Red,
        Level::Warn => Color::Yellow,
        Level::Info => Color::Green,
        Level::Debug => Color::Magenta,
        Level::Trace => Color::DarkGreen,
    }
}

fn get_level_text(level: Level) -> &'static str {
    match level {
        Level::Error => "ERROR",
        Level::Warn => "WARN ",
        Level::Info => "INFO ",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE",
    }
}