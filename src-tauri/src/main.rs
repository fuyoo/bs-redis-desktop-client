#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
extern crate log;
extern crate anyhow;

use tauri::{AppHandle, Manager, RunEvent, WindowEvent, Wry};

fn main() {
    if cfg!(debug_assertions) {
        env_logger::init();
    }
    fix_path_env::fix().expect("path error!");
    let _app = tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("发生了未知错误！");

    _app.run(|handle,ev| {
        match ev {
            RunEvent::Exit => {}
            RunEvent::ExitRequested { api,.. } => {
               // api.prevent_exit();
            }
            RunEvent::WindowEvent { label,event,.. } => {
                match event {
                    WindowEvent::Resized(_) => {}
                    WindowEvent::Moved(_) => {}
                    WindowEvent::CloseRequested { api,.. } => {
                    //api.
                    }
                    WindowEvent::Destroyed => {}
                    WindowEvent::Focused(_) => {}
                    WindowEvent::ScaleFactorChanged { scale_factor,new_inner_size,.. } => {

                    }
                    WindowEvent::FileDrop(_) => {}
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
