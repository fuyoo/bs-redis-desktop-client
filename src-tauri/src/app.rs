/*! # app初始化相关
> app 工作目录初始化
> 配置初始化等
 */
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
#[cfg(not(target_os = "windows"))]
use std::io::{Read, Write};
#[cfg(target_os = "windows")]
use std::ptr::null;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    result::Result::Ok,
};
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
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};
#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Threading::{CreateMutexW, OpenMutexW},
};
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;
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
    use tauri::{SystemTrayMenu, SystemTrayMenuItem};

    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("open_window", "Show"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrControl+Q")),
    )
}

/// 系统菜单
#[cfg(not(target_os = "windows"))]
pub fn create_app_menu() -> Menu {
    let submenu_gear = Submenu::new(
        "Gear",
        Menu::new()
            .add_native_item(MenuItem::SelectAll)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::Zoom)
            .add_native_item(MenuItem::EnterFullScreen)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    );

    let set = CustomMenuItem::new("set".to_string(), "Setting");
    let submenu_customer = Submenu::new("System", Menu::new().add_item(set));
    Menu::new()
        .add_submenu(submenu_gear)
        .add_submenu(submenu_customer)
}

/// 系统菜单
#[cfg(target_os = "windows")]
pub fn create_app_menu() -> Menu {
    Menu::new()
}

pub fn handle_event_app_menu_event(event: WindowMenuEvent<Wry>) {
    match event.menu_item_id() {
        // "exit" => {
        //     std::process::exit(0);
        // }
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
            "open_window" => {
                let window = app.get_window("main").unwrap();
                window.unminimize().unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            _ => {}
        },
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
            let _ = CreateMutexW(null(), true, "bs_redis_desktop_client@fuyoo");
        } else {
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

#[cfg(target_os = "windows")]
fn open_reg_key() -> std::io::Result<()> {
    // first find current user reg table
    let current_key = RegKey::predef(HKEY_CURRENT_USER);
    let wv2 = current_key.open_subkey(
        "Software\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
    );
    if let Ok(key) = wv2 {
        let res: std::io::Result<String> = key.get_value("pv");
        if let Ok(_) = res {
            return Ok(());
        }
    };
    // then find all account reg table
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let w2 = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}")?;
    let _: String = w2.get_value("pv")?;
    Ok(())
}

//windows下检查是否安装了WebView2
#[cfg(target_os = "windows")]
pub fn webview2_is_installed() {
    if let Err(_) = open_reg_key() {
        unsafe {
            MessageBoxW(
                None,
                "WebView2运行时未找到，点击确定按钮去安装吧！",
                "出错啦！",
                MB_OK,
            );
            let _ = tauri::api::shell::open(
                "https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/#download-section"
                    .to_string(),
                None,
            );
            std::process::exit(0);
        }
    };
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
