use std::sync::Mutex;

use crate::{api::resp::Response, r_ok};
use serde::{Deserialize, Serialize};
use tauri::{webview, AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, Result, Window};

static MAIN_WINDOW_ID: &str = "main";
static SETTING_VIEW_ID: &str = "settings";
#[derive(Default, Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Tab {
    pub id: String,
    pub name: Option<String>,
    pub active: Option<bool>,
    pub url: Option<String>,
}

#[tauri::command]
pub async fn tab_change(app: tauri::AppHandle, mut tab: Tab) -> Result<Response<()>> {
    println!("tab {:#?}", tab);
    // first we should check webview is created? if it has, show it hidden others.
    let view = app.app_handle().get_window(MAIN_WINDOW_ID).unwrap();
    // if name is none, we do not anything
    if tab.name.is_none() {
        return Ok(r_ok!((), None));
    }
    let hidden = |win: &Window, app: &AppHandle| -> Result<()> {
        // first set all tab active false
        if let Ok(mut tabs) = app.app_handle().state::<Mutex<Vec<Tab>>>().lock() {
            for item in tabs.iter_mut() {
                item.active = Some(item.id == tab.id)
            }
        }
        let views = win.webviews();
        for item in views {
            // hide other views
            item.hide()?;
        }
        Ok(())
    };
    // if webview is exist, show it.
    if let Some(wb) = view.get_webview(&tab.id) {
        hidden(&view, &app)?;
        // show self
        wb.show()?;
        // doesn't perform next code.
        return Ok(r_ok!((), None));
    }
    // add a new webview

    // first hide all webviews
    hidden(&view, &app)?;
    // start add a new webview
    let url = match tab.url.clone() {
        Some(url) => url.parse().map_err(|e| anyhow::Error::from(e))?,
        None => format!("/#/tab/{}/main/info", &tab.id)
            .parse()
            .map_err(|e| anyhow::Error::from(e))?,
    };

    let v = webview::WebviewBuilder::new(&tab.id, tauri::WebviewUrl::App(url)).auto_resize();
    let fac = view.current_monitor()?.unwrap().scale_factor();
    let size = view.inner_size()?.to_logical::<f64>(fac);

    view.add_child(
        v,
        LogicalPosition::<f64>::new(0., 0.),
        LogicalSize::new(size.width, size.height),
    )?;

    // push tab
    if tab.id != SETTING_VIEW_ID && tab.id != MAIN_WINDOW_ID {
        if let Ok(mut tabs) = app.app_handle().state::<Mutex<Vec<Tab>>>().lock() {
            tab.active = Some(true);
            tabs.push(tab.clone());
            _ = app.emit("update-tabs", ());
        }
    }

    Ok(r_ok!((), None))
}

#[tauri::command]
pub async fn tab_list(app: tauri::AppHandle) -> Result<Response<Vec<Tab>>> {
    let v = match app.app_handle().state::<Mutex<Vec<Tab>>>().lock() {
        Ok(tabs) => tabs.clone(),
        Err(_) => Vec::new(),
    };
    Ok(r_ok!(v, None))
}

#[tauri::command]
pub async fn tab_close(app: tauri::AppHandle, tab: Tab) -> Result<Response<bool>> {
    let mut tab_id = String::from(MAIN_WINDOW_ID);
    let handle: &AppHandle = app.app_handle();
    let state = handle.state::<Mutex<Vec<Tab>>>();
    // first update tab list
    if let Ok(mut tabs) = state.lock() {
        if let Some(pos) = tabs.iter().position(|item| item.id == tab.id) {
            tabs.remove(pos);
            // here emit update-tabs event to frontend
            _ = app.app_handle().emit("update-tabs", ());
        }
        // Find the next active tab or fallback logic
        if let Some(active_tab) = tabs.iter().find(|item| item.active.unwrap_or(false)) {
            tab_id = String::from(&active_tab.id);
        } else {
            match tabs.first_mut() {
                Some(tan) => {
                    tab_id = String::from(&tan.id);
                    tan.active = Some(true);
                }
                None => {}
            }
        }
    }

    // close webview
    if let Some(view) = app.app_handle().get_webview(&tab.id) {
        view.close()?;
    }
    // show a tab
    if let Some(view) = app.app_handle().get_webview(&tab_id) {
        view.show()?;
    }
    Ok(r_ok!(true, None))
}
