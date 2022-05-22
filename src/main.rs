#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate sciter;

use anyhow::{Error,anyhow};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let resource = include_bytes!("resource.rc");
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8		// Enables `Sciter.machineName()`.  Required for opening file dialog (`view.selectFile()`)
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8	// Enables opening file dialog (`view.selectFile()`)
    )).map_err(|e| anyhow!("{:?}",e))?;
    #[cfg(debug_assertions)]
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).map_err(|e| anyhow!("{:?}",e))?;
    let mut frame = sciter::Window::new();
    frame.set_title("fuyoo");
    #[cfg(debug_assertions)]
    if cfg!(target_os="macos") {
        frame.set_options(sciter::window::Options::DebugMode(true)).map_err(|e| anyhow!("{:?}",e))?;
    }
    frame.archive_handler(resource).map_err(|e| anyhow!("{:?}",e))?;
    #[cfg(not(target_os = "windows"))]
    frame.load_file("this://app/index.html");
    #[cfg(target_os = "windows")]
    frame.load_file("this://app/index_win.html");

    frame.run_app();
    Ok(())
}