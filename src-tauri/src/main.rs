// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
#[cfg(target_os = "windows")]
use window_shadows::set_shadow;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
