pub mod api;
pub mod tabs;
use std::collections::HashSet;

use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri::RunEvent;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default();
    // Access the system shell. Allows you to spawn child processes
    // and manage files and URLs using their default application.
    app = app.plugin(tauri_plugin_shell::init());
    // Save window positions and sizes and restore them when the app is reopened.
    app = app.plugin(tauri_plugin_window_state::Builder::default().build());
    app = app.setup(|_app| {
        // here mean's to adapt windows and linux native window frame controller.
        #[cfg(not(target_os = "macos"))]
        match _app.app_handle().get_webview_window("main") {
            None => {}
            Some(win) => {
                let _ = win.set_decorations(false);
            }
        }
        Ok(())
    });
    let runner = app
    .manage(Mutex::new(HashSet::<tabs::Tab>::new()))
    .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![api::request,
            tabs::tab_list,tabs::tab_change,tabs::tab_view_resize])
        .build(tauri::generate_context!())?;
    runner.run(|app, evt| match evt {
        RunEvent::Exit => {}
        RunEvent::ExitRequested { .. } => {}
        RunEvent::WindowEvent { .. } => {}
        RunEvent::Ready => match app.get_webview_window("main") {
            None => {}
            Some(win) => {
                let _ = win.show();
            }
        },
        _ => {}
    });
    Ok(())
}
