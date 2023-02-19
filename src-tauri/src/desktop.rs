use anyhow::Result;
use tauri::{RunEvent};
use crate::utils::init_sqlite;

pub fn main() -> Result<()> {
    let _app = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())?;

    _app.run(|_handle, _ev| {
        match _ev {
            RunEvent::Ready => {
                let _ = init_sqlite();
                println!("app is ready!");
            }
            _ => {}
        }
    });
    Ok(())
}