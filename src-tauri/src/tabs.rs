use std::sync::Mutex;

use crate::{api::resp::Response, r_ok};
use serde::{Deserialize, Serialize};
use tauri::{webview, Emitter, LogicalPosition, LogicalSize, Manager, Result};

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
pub async fn tab_change(app: tauri::AppHandle, tab: Tab) -> Result<Response<()>> {
    println!("tab {:#?}", tab);
    // first we should check webview is created? if it has, show it hidden others.
    let view = app.app_handle().get_window(MAIN_WINDOW_ID).unwrap();
    // if name is none, we do not anything
    if tab.name.is_none() {
        return Ok(r_ok!((), None));
    }
    let id = tab.id.clone();
    println!(
        "view.get_webview(&tab.id) {:?}",
        view.get_webview(&tab.id).is_some()
    );
    match view.get_webview(&tab.id) {
        None => {
            let url = match tab.url.clone() {
                Some(url) => url.parse().map_err(|e| anyhow::Error::from(e))?,
                None => format!("/#/tab/{}/main/info", &tab.id)
                    .parse()
                    .map_err(|e| anyhow::Error::from(e))?,
            };

            let v =
                webview::WebviewBuilder::new(&tab.id, tauri::WebviewUrl::App(url)).auto_resize();
            let fac = view.current_monitor()?.unwrap().scale_factor();
            let size = view.inner_size()?.to_logical::<f64>(fac);
            view.add_child(
                v,
                LogicalPosition::<f64>::new(0., 0.),
                LogicalSize::new(size.width, size.height),
            )?;
            // here we hide other views
            let vbs = view.webviews();
            for item in vbs {
                if item.label() != &tab.id {
                    item.hide()?;
                }
                // open devtools
                #[cfg(debug_assertions)]
                item.open_devtools();
            }
        }
        Some(vb) => {
            let vbs = view.webviews();
            for item in vbs {
                if item.label() != &tab.id {
                    // todo if this error maybe webview has been destory.
                    let _ = item.hide();
                }
            }
            vb.show()?;
        }
    }
    let v = match app.app_handle().state::<Mutex<Vec<Tab>>>().lock() {
        Ok(mut tabs) => {
            if tab.id == SETTING_VIEW_ID || tab.id == MAIN_WINDOW_ID {
                ()
            } else if !tabs.contains(&tab) {
                tabs.push(tab);
                // here emit update-tabs event to frontend
                _ = app.app_handle().emit("update-tabs", ());
            }
            ()
        }
        Err(_) => (),
    };
    // set active
    match app.app_handle().state::<Mutex<Vec<Tab>>>().lock() {
        Ok(mut tabs) => {
            for item in tabs.iter_mut() {
                item.active = Some(item.id == id)
            }
        }
        Err(_) => (),
    };

    Ok(r_ok!(v, None))
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
    let v = match app.app_handle().state::<Mutex<Vec<Tab>>>().lock() {
        Ok(mut tabs) => {
            if let Some(pos) = tabs.iter().position(|item| item.id == tab.id) {
                tabs.remove(pos);
                true
            } else {
                false
            }
        }
        Err(_) => false,
    };
    if v {
        // here emit update-tabs event to frontend
        _ = app.app_handle().emit("update-tabs", ());
    }
    if let Some(view) = app.app_handle().get_webview(&tab.id) {
        view.close()?;
    }
    // focus a tab.
    match app.app_handle().state::<Mutex<Vec<Tab>>>().lock() {
        Ok(mut tabs) => {
            let mut has = false;
            for item in tabs.iter_mut() {
                if let Some(active) = item.active {
                    if active {
                       has = true;
                       break;
                    }
                }
            }
            if !has {
                // has more active tab.
                if let Some(item) = tabs.last_mut() {
                    item.active = Some(true);
                    if let Some(view) = app.app_handle().get_webview(&tab.id) {
                        view.show()?;
                    }
                } else {
                    // no have any more active tab,show main window.
                    if let Some(view) = app.app_handle().get_webview(MAIN_WINDOW_ID) {
                        view.show()?;
                    }
                }
            }
        }
        Err(_) => (),
    };
    Ok(r_ok!(v, None))
}
