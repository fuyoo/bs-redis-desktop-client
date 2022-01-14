/*! # app初始化相关
> app 工作目录初始化
> 配置初始化等
 */
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
#[cfg(target_os = "windows")]
use std::ptr::null;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    result::Result::Ok,
};
#[cfg(not(target_os = "windows"))]
use std:: io::{Read, Write};
use tauri::{
    command, AppHandle, CustomMenuItem, Event, Manager, Menu, SystemTray, SystemTrayEvent, Window,
    WindowMenuEvent, Wry,
};

#[cfg(not(target_os = "windows"))]
use tauri::{MenuItem, Submenu};
#[cfg(target_os = "windows")]
use tauri::{SystemTrayMenu, SystemTrayMenuItem};
use tokio::net::UdpSocket;
#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Threading::{OpenMutexW,CreateMutexW},
};

lazy_static! {
    pub static ref APP: Mutex<App> = Mutex::new(App::new());
}

///初始化app文件夹
pub fn init_app_dir() -> bool {
    if !Path::new(&crate::app::APP.lock().app_dir).exists() {
        if let Ok(_) = create_dir_all(&crate::app::APP.lock().app_dir) {
            return true;
        }
        return false;
    }
    true
}

/// app配置map
pub struct App {
    pub app_dir: PathBuf,
}

impl App {
    pub fn new() -> App {
        let cfg = tauri::Config::default();
        match tauri::api::path::app_dir(&cfg) {
            None => App {
                app_dir: PathBuf::new(),
            },
            Some(p) => App {
                app_dir: p.join("com.echosocket.bs"),
            },
        }
    }
}

///响应结构
#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub data: T,
    pub msg: String,
}

impl<T> Response<T> {
    pub fn new(code: i32, data: T, msg: &str) -> Response<T> {
        Response {
            code,
            data,
            msg: msg.to_string(),
        }
    }

    pub fn ok(data: T, msg: &str) -> Self {
        Self::new(200, data, msg)
    }

    pub fn err(data: T, msg: &str) -> Self {
        Self::new(300, data, msg)
    }
}

/// 创建任务栏图标
#[cfg(target_os = "windows")]
pub fn create_try() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let set = CustomMenuItem::new("set".to_string(), "设置");
    let tray_menu = SystemTrayMenu::new()
        .add_item(set)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

/// 创建任务栏图标
#[cfg(not(target_os = "windows"))]
pub fn create_try() -> SystemTray {
    SystemTray::new()
}

/// 系统菜单
#[cfg(not(target_os = "windows"))]
pub fn create_app_menu() -> Menu {
    let quit = CustomMenuItem::new("set".to_string(), "设置");
    let close = CustomMenuItem::new("exit".to_string(), "退出");
    let submenu = Submenu::new("软件", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(submenu);
    menu
}

/// 系统菜单
#[cfg(target_os = "windows")]
pub fn create_app_menu() -> Menu {
    Menu::new()
}

pub fn handle_event_app_menu_event(event: WindowMenuEvent<Wry>) {
    match event.menu_item_id() {
        "exit" => {
            std::process::exit(0);
        }
        "set" => {
            event.window().emit("set", "").unwrap();
        }
        _ => {
            print!("at handle_event_app_menu_event")
        }
    }
}

/// 任务栏图标点击事件
pub fn handle_system_tray_event(app: &AppHandle<Wry>, e: SystemTrayEvent) {
    match e {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "set" => {
                app.get_window("main").unwrap().emit("set", "").unwrap();
            }
            _ => {}
        },
        SystemTrayEvent::LeftClick { .. } => {
            if let Some(window) = app.get_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
                info!("handle_system_tray_event at here?");
            }
        }
        _ => {}
    }
}

/// 监听app事件
pub fn handle_app_event(app_handle: &AppHandle<Wry>, event: Event) {
    match event {
        Event::CloseRequested { label, api, .. } => {
            if label == "main" {
                let app_handle = app_handle.clone();
                app_handle.get_window(&label).unwrap().hide().unwrap();
                // use the exposed close api, and prevent the event loop to close
                api.prevent_close();
            }
        }
        _ => {}
    }
}

/// 创建pid文件
#[cfg(not(target_os = "windows"))]
pub fn crete_pid_file() {
    let pid = std::path::PathBuf::from(&APP.lock().app_dir).join("app.pid");
    let id = std::process::id();
    let mut fd = std::fs::File::create(pid).unwrap();
    let _ = fd.write_all(format!("{}", id).as_bytes()).unwrap();
}
/// 锁定单例模式 windows
#[cfg(target_os = "windows")]
pub fn lock_single() {
    unsafe {
        let _ = OpenMutexW(0, true, "bs_redis_desktop_client@fuyoo");
        let WIN32_ERROR(code) = GetLastError();
        if code == 2 {
            // 创建锁
            let _ =  CreateMutexW(null(),true,"bs_redis_desktop_client@fuyoo");
        }  else {
            // 已经存在了，退出
            send_wake_up();
            std::process::exit(0);
        }
    }
}

/// 锁定单例模式 linux
#[cfg(not(target_os = "windows"))]
pub fn lock_single() {
    // check pid file is exists?
    let pid = std::path::PathBuf::from(&APP.lock().app_dir).join("app.pid");
    // pid is not exists? create pid file an start app
    if !pid.exists() {
        return;
    }
    // pid is exists? check the app is running?
    let fd = std::fs::File::open(pid);
    let mut data = vec![];
    fd.unwrap().read_to_end(&mut data).unwrap();
    let fd = String::from_utf8(data).unwrap().parse().unwrap();
    unsafe {
        // send a signal to check process is running?
        let status = libc::kill(fd, 0);
        // running ?
        if status == 0 {
            send_wake_up();
            std::process::exit(0);
        } else {
            // not running create a pid file
            crete_pid_file()
        }
    }
}
/// 发送拉起请求
fn send_wake_up() {
    tauri::async_runtime::block_on(async {
        let res = UdpSocket::bind("127.0.0.1:24253").await.unwrap();
        let mut data = [0u8; 16];
        for i in 0..16 {
            data[i] = 1 as u8
        }
        res.send_to(&data, "127.0.0.1:24254").await.unwrap();
    });
}

#[command]
pub async fn listen_single(window: Window) {
    let _: tauri::async_runtime::JoinHandle<anyhow::Result<(), anyhow::Error>> =
        tauri::async_runtime::spawn(async move {
            let socket = UdpSocket::bind("127.0.0.1:24254").await?;
            loop {
                let mut buf = [0; 32];
                let (size, _) = socket.recv_from(&mut buf).await?;
                if size != 16 {
                    return Ok(());
                };
                // check status
                let mut status = true;
                for item in &buf[0..size] {
                    if *item as i32 != 1 {
                        status = false;
                        break;
                    }
                }
                if status {
                    let _ = window.emit_all("single", "");
                };
            }
        });
}
