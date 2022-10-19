use anyhow::Result;
use window_shadows::set_shadow;
use tauri::{Manager, RunEvent};
use crate::utils::init_sqlite;

pub fn main() -> Result<()> {
    let _app = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            #[cfg(target_os = "windows")]
            if let Some(window) = app.get_window("main") {
                let _ = set_shadow(&window, true);
            };
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())?;

    _app.run(|_handle, _ev| {
        match _ev {
            RunEvent::Ready => {
                let _ = init_sqlite();
                println!("system is ready!");
            }
            _ => {}
        }
    });
    Ok(())
}