pub mod api;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![api::request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
