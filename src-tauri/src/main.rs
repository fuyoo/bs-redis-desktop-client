#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod utils;

extern crate log;
extern crate anyhow;

use log::{info};
use window_shadows::set_shadow;

use tauri::{AppHandle, FileDropEvent, Manager, RunEvent, WindowEvent, Wry};
use crate::utils::make_sure_single_case;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::init();
    }
    make_sure_single_case();
    fix_path_env::fix().expect("path error!");
    let _app = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("发生了未知错误！");

    _app.run(|handle, ev| {
        match ev {
            RunEvent::Exit => {}
            RunEvent::ExitRequested { api, .. } => {
                // here to handle exit status
                api.prevent_exit();
            }
            RunEvent::WindowEvent { label, event, .. } => {
                match event {
                    WindowEvent::Resized(_) => {}
                    WindowEvent::Moved(_) => {}
                    WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close()
                    }
                    WindowEvent::Destroyed => {}
                    WindowEvent::Focused(_) => {}
                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size, .. } => {}
                    WindowEvent::FileDrop(evt) => {
                        match evt {
                            FileDropEvent::Hovered(data) => {
                                info!("{:#?}",data)
                            }
                            FileDropEvent::Dropped(data) => {
                                info!("{:#?}",data)
                            }
                            FileDropEvent::Cancelled => {}
                            _ => {}
                        }
                    }
                    WindowEvent::ThemeChanged(_) => {}
                    _ => {}
                }
            }
            RunEvent::Ready => {}
            RunEvent::Resumed => {}
            RunEvent::MainEventsCleared => {}
            _ => {}
        };
    });
}
