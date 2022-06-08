#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod events;
mod logger;
mod routes;
mod sqlite;

extern crate sciter;

use crate::events::{Action, Evt};
use crate::logger::open_log_file;
use anyhow::{ Result};
use std::io::Write;
use crate::app::make_sure_single_case;

#[tokio::main]
async fn main() {
    match run() {
        Err(e) => match open_log_file() {
            Ok(mut file) => {
                let _ = file.write_all(format!("{}\n", e).as_bytes());
            }
            Err(_) => {}
        },
        _ => {}
    }
}

fn run() -> Result<()> {
    logger::init()?;
    make_sure_single_case();
    app::create_main()?;
    Ok(())
}
