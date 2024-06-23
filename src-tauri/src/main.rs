// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;

use tauri::Manager;
#[cfg(target_os = "windows")]
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
              let window = app.get_window("main").unwrap();
                let _ = window.set_decorations(false);
              set_shadow(&window, true).expect("Unsupported platform!");  
            }
            #[cfg(target_os = "macos")]
            {
            }
            Ok(())
        })
        .on_page_load(|w,_| {
            let _ = w.show();
        })
        .invoke_handler(tauri::generate_handler![api::request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
