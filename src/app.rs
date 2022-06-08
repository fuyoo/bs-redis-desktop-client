use crate::logger::error;
use crate::{routes, sqlite, Action, Evt};
use anyhow::{anyhow, Error, Result};
use flume::{Receiver, RecvError};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use sciter::{make_args, Window};
use std::fs::DirBuilder;
use std::path::PathBuf;
use std::ptr::null;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Threading::{CreateMutexW, OpenMutexW},
};

/// create
pub fn app() -> &'static Mutex<App> {
    static INSTANCE: OnceCell<Mutex<App>> = OnceCell::new();
    INSTANCE.get_or_init(|| Mutex::new(App::new()))
}

/// app globe data
#[derive(Debug)]
pub struct App {
    pub app_data_dir: PathBuf,
}

impl App {
    pub fn new() -> Self {
        App {
            app_data_dir: PathBuf::new(),
        }
    }
    pub fn set_app_data_dir(&mut self, dir: PathBuf) {
        self.app_data_dir = dir
    }
}

/// create main window
pub fn create_main() -> Result<(), Error> {
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8        // Enables `Sciter.machineName()`.  Required for opening file dialog (`view.selectFile()`)
            | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8, // Enables opening file dialog (`view.selectFile()`)
    ))
    .map_err(|e| anyhow!("{:?}", e))?;
    // at rust side, open the sciter debug mode
    #[cfg(debug_assertions)]
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true))
        .map_err(|_| anyhow!("unknown error"))?;
    // include sciter packfolder tool crated the resource file
    let resource = include_bytes!("resource.rc");
    // create window
    let mut window = sciter::WindowBuilder::main_window()
        .with_size((1024, 600))
        .create();
    // load resource file
    window
        .archive_handler(resource)
        .map_err(|e| anyhow!("{:?}", e))?;
    // generate channel to dispatch data
    let (sender, receiver) = flume::unbounded();
    // listen channel
    do_request_services(receiver);
    // inject event
    window.event_handler(Evt { sender });
    // load ui
    if cfg!(target_os = "windows") {
        window.load_file("this://app/index_win.html");
    } else {
        window.load_file("this://app/index.html");
    }
    init_app_directory(&window)?;
    sqlite::init()?;
    // start event-loop
    window.run_app();
    Ok(())
}

fn init_app_directory(window: &Window) -> Result<()> {
    let val = window
        .get_host()
        .call_function("get_env_path", &make_args!("appdata"))
        .map_err(|e| anyhow!("{:?}", e))?;
    let dir = val.as_string();
    if dir.is_none() {
        return Err(anyhow!("get appdata directory failed"));
    }
    // join the app directory
    let path = PathBuf::from(dir.unwrap()).join("Rd");
    // save to globe data
    app().lock().set_app_data_dir(path.clone());
    // create dir
    if !path.exists() {
        DirBuilder::new().create(path)?;
    }
    Ok(())
}

fn do_request_services(receiver: Receiver<Action>) {
    tokio::spawn(async move {
        loop {
            match receiver.recv_async().await {
                Ok(action) => match routes::dispatch(action).await {
                    Ok(_) => {}
                    Err(err) => error!("{}", err),
                },
                Err(e) => match e {
                    RecvError::Disconnected => {
                        error!("do_request_services channel disconnected");
                        std::process::exit(0);
                    }
                },
            }
        }
    });
}

/// make sure single case
#[cfg(target_os = "windows")]
pub fn make_sure_single_case() {
    unsafe {
        let _ = OpenMutexW(0, true, "bs.echosocket.com@fuyoo");
        let WIN32_ERROR(code) = GetLastError();
        if code == 2 {
            // 创建锁
            let _ = CreateMutexW(null(), true, "bs.echosocket.com@fuyoo");
        } else {
            error!("app is running, please do not run again");
            std::process::exit(0);
        }
    }
}

/// make sure single case
#[cfg(not(target_os = "windows"))]
pub fn make_sure_single_case() {
    todo!();
}
