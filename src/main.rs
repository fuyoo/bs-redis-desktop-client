#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod events;
mod routes;
mod logger;

extern crate sciter;

use std::io::Write;
use flume::{Receiver, RecvError};
use anyhow::{anyhow, Error, Result};
use logger::info;
use crate::events::{Action, Evt};
use crate::logger::open_log_file;

#[tokio::main]
async fn main() {
    match run() {
        Err(e) => {
            match open_log_file() {
                Ok(mut file) => {
                    let _ = file.write_all(format!("{}\n", e).as_bytes());
                }
                Err(_) => {}
            }
        }
        _ => {}
    }
    info!("app exited!");
}

fn run() -> Result<()> {
    logger::init()?;
    create_main()?;
    Ok(())
}

fn create_main() -> Result<(), Error> {
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8        // Enables `Sciter.machineName()`.  Required for opening file dialog (`view.selectFile()`)
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8    // Enables opening file dialog (`view.selectFile()`)
    )).map_err(|e| anyhow!("{:?}",e))?;
    #[cfg(debug_assertions)]
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).map_err(|_|{anyhow!("unknown error")})?;
    let resource = include_bytes!("resource.rc");
    let mut window = sciter::WindowBuilder::main_window()
        .with_size((1024, 600))
        .create();
    window.archive_handler(resource).map_err(|e| anyhow!("{:?}",e))?;
    let (sender, reciver) = flume::unbounded();
    do_request_services(reciver);
    window.event_handler(Evt { sender });
    if cfg!(target_os = "windows") {
        window.load_file("this://app/index_win.html");
    } else {
        window.load_file("this://app/index.html");
    }
    window.run_app();
    Ok(())
}

fn do_request_services(receiver: Receiver<Action>) {
    tokio::spawn(async move {
        loop {
            let res = receiver.recv_async().await;
            match res {
                Ok(action) => {
                    match routes::dispatch(action).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("err {}", err);
                        }
                    }
                }
                Err(e) => {
                    match e {
                        RecvError::Disconnected => {
                            break;
                        }
                    }
                }
            }
        }
    });
}