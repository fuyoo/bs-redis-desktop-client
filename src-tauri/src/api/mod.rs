use std::future::Future;

use super::tools::extract;
use crate::api::rdb::{ConnectionImpl, RedisClientImpl};
use crate::api::resp::Response;
use redis::{cmd, Value};
use serde::{Deserialize, Serialize};
use tauri::{command, Emitter, Manager, Result};
pub mod rdb;
pub mod resp;
use crate::{r_404, r_error, r_ok};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};

async fn route<T: serde::Serialize>(f: impl Future<Output = Result<Response<T>>>) -> String {
    match f.await {
        Ok(r) => r.to_string(),
        Err(e) => r_error!(e).to_string(),
    }
}

#[command]
pub async fn request(
    path: &str,
    connection_info: rdb::ConnectionImpl,
    data: &str,
) -> Result<String> {
    let r = match path {
        "/cmd" => route(do_query(connection_info, data)).await,
        // checking connection status
        "/status" => route(status(connection_info)).await,
        // fetch redis info
        "/info" => route(info(connection_info, data)).await,
        &_ => r_404!(path).to_string(),
    };
    Ok(r)
}

// check connection status
async fn status(connection_info: ConnectionImpl) -> Result<Response<Option<String>>> {
    let r = connection_info
        .into_client()?
        .do_command::<Option<String>>(&cmd("ping"))
        .await?;
    Ok(r_ok!(r, None))
}

// base_info
async fn info(connection_info: ConnectionImpl, data: &str) -> Result<Response<Option<String>>> {
    if data == "" {
        let r = connection_info
            .into_client()?
            .do_command::<Option<String>>(&cmd("info"))
            .await?;
        return Ok(r_ok!(r, None));
    }
    let r = connection_info
        .into_client()?
        .do_command::<Option<String>>(&cmd("info").arg(data))
        .await?;
    Ok(r_ok!(r, None))
}

/// secarch keys param.
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyParam {
    pub cursor: Option<usize>,
    pub key: String,
    pub count: Option<usize>,
}

// here,we provide a query function,to do all query from frontend.
async fn do_query(connection_info: ConnectionImpl, data: &str) -> Result<Response<String>> {
    let param = extract::<Vec<String>>(data)?;
    // just print log in debug environment
    #[cfg(debug_assertions)]
    println!("cmd param: {:#?}", param);
    
    let mut cmd_ = cmd(param.get(0).unwrap());
    for (_, v) in param.iter().skip(1).enumerate() {
        cmd_.arg(v);
    }
    let client = connection_info.into_client()?;
    let resp = client.do_command::<Value>(&mut cmd_).await?;
    Ok(r_ok!(rdb::convert_to_string(resp)?, None))
}

#[derive(Serialize, Clone, Debug)]
pub struct Evt {
    pub evt: String,
    pub data: String,
}

#[command]
pub async fn emit_event(app: tauri::AppHandle, evt: String, data: String) -> Result<Response<()>> {
    app.app_handle().emit("emit-event", Evt { evt, data })?;
    Ok(r_ok!((), None))
}

#[command]
pub async fn quit(app: tauri::AppHandle) -> Result<Response<()>> {
    app.app_handle().exit(0);
    Ok(r_ok!((), None))
}
#[command]
pub async fn init_tray(app: tauri::AppHandle, text: String) -> Result<Response<()>> {
    let app = app.app_handle();
    let quit_i = MenuItem::with_id(app, "quit", text, true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;
    if let Some(tray) = app.tray_by_id("1") {
        tray.set_menu(Some(menu))?;
        return Ok(r_ok!((), None));
    }
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click { button, .. } => match button {
                MouseButton::Left => {
                    #[cfg(target_os = "macos")]
                    let _ = tauri::AppHandle::show(tray.app_handle());
                    #[cfg(not(target_os = "macos"))]
                    {
                        let win = tray
                            .app_handle()
                            .get_webview_window("main")
                            .expect("no main window");
                        win.show().expect("failed to show window");
                        let _ = win.set_focus();
                        let _ = win.unminimize();
                    }
                }
                _ => {}
            },
            _ => {}
        });
    println!("{:?}", _tray.id());
    // 设置图标
    #[cfg(target_os = "macos")]
    let icon = include_bytes!("../../icons/template.png");
    #[cfg(target_os = "macos")]
    let img = tauri::image::Image::from_bytes(icon)?;
    #[cfg(target_os = "macos")]
    let _tray = _tray.icon(img);
    #[cfg(target_os = "macos")]
    let _tray = _tray.icon_as_template(true);
    #[cfg(not(target_os = "macos"))]
    let icon = include_bytes!("../../icons/logo.png");
    #[cfg(not(target_os = "macos"))]
    let img = tauri::image::Image::from_bytes(icon)?;
    #[cfg(not(target_os = "macos"))]
    let _tray = _tray.icon(img);
    _tray.build(app)?;
    Ok(r_ok!((), None))
}
