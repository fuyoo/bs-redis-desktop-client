use std::env;
use std::fs::{File, DirBuilder};
pub use log::{info, warn, error, trace, debug};
use env_logger::{Env, Builder};
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use env_logger::fmt::{Color, Formatter};
use log::{Level, Record};

/// init logger
pub fn init() -> Result<()> {
    Builder::from_env(Env::default()
        .default_filter_or("info"))
        .format(format_log)
        .try_init()?;
    Ok(())
}

/// format log file
fn format_log(buf: &mut Formatter, record: &Record) -> std::io::Result<()> {
    if !cfg!(debug_assertions) {
        if record.level() == Level::Error {
            match save_log_to_file(&record) {
                _ => {}
            }
        };
        return Ok(());
    }
    let color = match record.level() {
        Level::Error => {
            Color::Red
        }
        Level::Warn => {
            Color::Rgb(255, 125, 0)
        }
        Level::Info => {
            Color::Green
        }
        Level::Debug => {
            Color::Rgb(88, 88, 88)
        }
        Level::Trace => {
            Color::Rgb(88, 88, 88)
        }
    };
    let mut style = buf.style();
    style.set_color(color).set_bold(true);
    match writeln!(buf, "{}:{} {:<5} {}",
                   record.file().unwrap_or("unnamed"),
                   record.line().unwrap_or(0),
                   style.value(record.level()),
                   style.value(record.args())) {
        _ => {
            Ok(())
        }
    }
}

/// save app error log to log file
fn save_log_to_file(record: &Record) -> Result<()> {
    let mut file = open_log_file()?;
    let str = format!("{}:{} {:<5} {}\n",
                      record.file().unwrap_or("unnamed"),
                      record.line().unwrap_or(0),
                      record.level(),
                      record.args());
    file.write_all(str.as_bytes())?;
    Ok(())
}

pub fn open_log_file() -> std::io::Result<File> {
    let mut log_dir = PathBuf::from(env::current_dir()?);
    log_dir = log_dir.join("log");
    if !log_dir.exists() {
        DirBuilder::new().recursive(true).create(&log_dir)?; // (&log_dir)?;
    }
    let log_file = log_dir.join("app.log");
    Ok(File::options().create(true).append(true).open(log_file)?)
}