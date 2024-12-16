use std::{collections::HashSet, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::{webview, LogicalPosition, LogicalSize, Manager, Result};

use crate::{api::resp::{ Response}, r_ok};
#[derive(Default,Serialize,Deserialize,Hash, Eq, PartialEq, Debug, Clone)]
pub struct Tab{
pub id:String,
pub name:Option<String>
}

#[tauri::command]
pub async fn tab_change(app: tauri::AppHandle,tab:Tab) -> Result<()> {
  // first we should check webview is created? if it has, show it hidden others.
    let view = app.app_handle().get_window("main").unwrap();
    match  view.get_webview(&tab.id) {
        None => {
            let v = webview::WebviewBuilder::new(&tab.id, 
                tauri::WebviewUrl::App(format!("/#/host/{}/status",&tab.id)
                .parse().unwrap()));
            let size = view.inner_size()?;
            view.add_child(
                v,
                LogicalPosition::new(0,60),
                LogicalSize::new(size.width, size.height - 60)
            )?;
            // here we hide other views
            let vbs = view.webviews();
            for item in  vbs {
                if item.label() != &tab.id && item.label() != "main" {
                    item.hide()?;
                }
            }
        },
        Some(vb) => {
           
            let vbs = view.webviews();
            for item in  vbs {
                if item.label() != &tab.id && item.label() != "main" {
                    // todo if this error maybe webview has been destory.
                   let _ = item.hide();
                }
                println!("{}",item.label())
            }
             // if tab id is main,we do not anything.
             if tab.id == "main" {
                return Ok(());
            }
            vb.set_position(LogicalPosition::new(0, 60))?;
            let size = view.inner_size()?;
            vb.set_size(LogicalSize::new(size.width, size.height - 60))?;
            vb.show()?;
        },  
    }
  let v = match app.app_handle().state::<Mutex<HashSet<Tab>>>().lock() {
    Ok(mut set) => {
        set.insert(tab);
    },
    Err(_) => (),
};
 Ok(())
}

#[tauri::command]
pub async fn tab_list(app: tauri::AppHandle) -> Result<Response<Vec<Tab>>> {
    let v = match app.app_handle().state::<Mutex<HashSet<Tab>>>().lock() {
        Ok(set) => set.clone().into_iter().collect::<Vec<Tab>>(),
        Err(_) => Vec::new(),
    };
    Ok(r_ok!(v, None))
}

#[tauri::command]
pub async fn tab_close(app: tauri::AppHandle,tab:Tab) -> Result<Response<bool>> {
    let v = match app.app_handle().state::<Mutex<HashSet<Tab>>>().lock() {
        Ok(mut set) => set.remove(&tab),
        Err(_) => false,
    };
    if let Some(view) = app.app_handle().get_webview(&tab.id) {
        view.close()?;
    }

    Ok(r_ok!(v, None))
}

#[tauri::command]
pub async fn tab_view_resize(app: tauri::AppHandle,id:String) -> Result<Response<bool>> {
      // if tab id is main,we do not anything.
      if &id == "main" {
            return Ok(r_ok!(true, None));
        }
    if let Some(vb) = app.app_handle().get_webview(&id) {
        // first we should check webview is created? if it has, show it and hidden others.
        let view = app.app_handle().get_window("main").unwrap();
        vb.set_position(LogicalPosition::new(0, 60))?;
            let size = view.inner_size()?;
            vb.set_size(LogicalSize::new(size.width, size.height - 60))?;
            vb.show()?;
    }
    Ok(r_ok!(true, None))
}
