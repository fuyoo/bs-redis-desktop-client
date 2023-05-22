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
        RunEvent::WindowEvent { event, label, .. } => match event {
            WindowEvent::CloseRequested { api, .. } => {
                // prevent close
                if let Some(win) = &_handle.get_window(&label) {
                    #[cfg(target_os = "macos")]
                    {
                        // on macos, we hide the window instead of close the window.
                        // because when we clicking the dock icon, we can show it again.
                        let _ = tauri::AppHandle::hide(&win.app_handle());
                        api.prevent_close();
                    }
                }
            }
            _ => {}
        },
        _ => {}
    });
    Ok(())
}
