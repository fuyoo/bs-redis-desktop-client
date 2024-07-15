pub mod api;

pub fn run() {
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
    app.invoke_handler(tauri::generate_handler![api::request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
