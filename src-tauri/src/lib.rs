pub mod api;
pub mod tabs;
pub mod tools;
use std::sync::Mutex;
use tauri::webview::PageLoadEvent;
use tauri::Manager;
use tauri::RunEvent;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};
fn build_try(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "关闭", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click { button, .. } => match button {
                        MouseButton::Left => {
                            #[cfg(target_os = "macos")]
                            let _ = tauri::AppHandle::show(tray.app_handle());
                            #[cfg(not(target_os = "macos"))]
                            {
                             let win = tray.app_handle()
                                .get_webview_window("main")
                                .expect("no main window");
                                win.show()
                                .expect("failed to show window");
                               let _ = win.set_focus();
                               let _ = win.unminimize();
                            }
                           
                        }
                        _ => {}
                    },
                    _ => {}
                });
            // 设置图标
            #[cfg(target_os = "macos")]
            let icon = include_bytes!("../icons/template.png");
            #[cfg(target_os = "macos")]
            let img = tauri::image::Image::from_bytes(icon)?;
            #[cfg(target_os = "macos")]
            let _tray = _tray.icon(img);
            #[cfg(target_os = "macos")]
            let _tray = _tray.icon_as_template(true);
             #[cfg(not(target_os = "macos"))]
            let icon = include_bytes!("../icons/icon.png");
             #[cfg(not(target_os = "macos"))]
            let img = tauri::image::Image::from_bytes(icon)?;
            #[cfg(not(target_os = "macos"))]
            let _tray = _tray.icon(img);
            _tray.build(app)?;
            Ok(())
}
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default();
    // Access the system shell. Allows you to spawn child processes
    // and manage files and URLs using their default application.
    app = app.plugin(tauri_plugin_shell::init());
    // use log plugin
    app = app.plugin(
        tauri_plugin_log::Builder::new()
            .target(
                #[cfg(debug_assertions)]
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                #[cfg(not(debug_assertions))]
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                }),
            )
            .build(),
    );
    // listen window event
    app = app.on_window_event(|win, evt| match evt {
        tauri::WindowEvent::CloseRequested { api, .. } => match win.label() {
            "main" => {
                api.prevent_close();
                #[cfg(target_os = "macos")]
                let _ = tauri::AppHandle::hide(win.app_handle());
                #[cfg(not(target_os = "macos"))]
                let _ = win.hide();
            }
            _ => {}
        },
        _ => {}
    });
    app = app.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
        let win = app.get_webview_window("main").expect("no main window");
        let _ = win.show();
        let _ = win.set_focus();
    }));
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
        build_try(_app)?;

        Ok(())
    });

    app = app
        .manage(Mutex::new(Vec::<tabs::Tab>::new()))
        .invoke_handler(tauri::generate_handler![
            api::request,
            tabs::tab_list,
            tabs::tab_change,
            tabs::tab_close
        ]);
    let app = app.on_page_load(|wb, evt| match evt.event() {
        PageLoadEvent::Finished => {
            let win = wb.window();
            if win.label() == "main" {
                win.show().expect("failed to show window");
                let _ = win.set_focus();
            }
        }
        _ => {}
    });
    let runner = app.build(tauri::generate_context!())?;
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
        // here we reactive macos dock click event
        #[cfg(target_os = "macos")]
        RunEvent::Reopen {
            has_visible_windows,
            ..
        } => {
            println!("Reopen event: {}", has_visible_windows);
            let _ = tauri::AppHandle::show(app);
        }
        _ => {}
    });
    Ok(())
}
