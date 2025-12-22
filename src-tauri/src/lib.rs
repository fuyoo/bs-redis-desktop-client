pub mod api;
pub mod tabs;
pub mod tools;
use std::sync::Mutex;
use tauri::webview::PageLoadEvent;
use tauri::Manager;
use tauri::RunEvent;
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build());
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
        };

        Ok(())
    });

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
    app = app.manage(Mutex::new(Vec::<tabs::Tab>::new()));
    app = app.invoke_handler(tauri::generate_handler![
        api::request,
        tabs::tab_list,
        tabs::tab_change,
        tabs::tab_close,
        api::emit_event,
        api::quit,
        api::init_tray,
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

    #[cfg(debug_assertions)]
    let runner = app.build(tauri::generate_context!())?;
    #[cfg(not(debug_assertions))]
    let runner = app.build(tauri::generate_context!("../tauri.conf.release.json"))?;
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
