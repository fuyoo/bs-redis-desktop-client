use anyhow::Result;

use tauri::RunEvent;

use crate::{routes, utils::init_sqlite};

pub fn main() -> Result<()> {
    let _app = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![routes::routes])
        .build(tauri::generate_context!())?;
    init_sqlite()?;
    _app.run(move |_handle, _ev| match _ev {
        RunEvent::Ready => {
            println!("app is ready!");
        }
        RunEvent::ExitRequested { api,.. } => api.prevent_exit(),
        _ => {}
    });
    Ok(())
}
