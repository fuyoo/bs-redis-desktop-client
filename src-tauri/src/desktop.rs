use anyhow::Result;

use tauri::{Manager, RunEvent, WindowEvent};

use crate::{routes, utils::init_sqlite};

pub fn main() -> Result<()> {
    let _app = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![routes::routes, routes::exit])
        .build(tauri::generate_context!())?;
    init_sqlite()?;
    _app.run(move |_handle, _ev| match _ev {
        RunEvent::Ready => {
            println!("app is ready!");
        }
        RunEvent::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested { api, .. } => {
                // prevent close for clear opened tab tabs
                api.prevent_close();
                if let Some(win) = &_handle.get_window("main") {
                    let _ = win.emit("clear", ());
                }
            }
            _ => {}
        },
        _ => {}
    });
    Ok(())
}
