use anyhow::{anyhow, Result};
use sqlx::{Connection, SqliteConnection};
use tauri::api::path;
#[cfg(target_os = "windows")]
use windows::{
    Win32::{
        Foundation::{GetLastError, WIN32_ERROR},
        Globalization::GetUserDefaultUILanguage,
        System::Threading::{CreateMutexW, OpenMutexW, SYNCHRONIZATION_ACCESS_RIGHTS},
        UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
    },
    w,
};

/// make sure app running at single-case
#[cfg(target_os = "windows")]
pub fn make_sure_single_case() {
    unsafe {
        let _ = OpenMutexW(
            SYNCHRONIZATION_ACCESS_RIGHTS(0),
            true,
            w!("link.xsa.bs@fuyoo"),
        );
        let WIN32_ERROR(code) = GetLastError();
        if code == 2 {
            // generate lock
            let _ = CreateMutexW(None, true, w!("link.xsa.bs@fuyoo"));
        } else {
            // get local language
            let lang = GetUserDefaultUILanguage();
            // Adapt to Chinese and english
            if lang == 2052u16 {
                MessageBoxW(
                    None,
                    w!("Rd 正在运行中, 请不要重复运行！"),
                    w!("提示"),
                    MB_OK,
                );
            } else {
                MessageBoxW(
                    None,
                    w!("Rd is Running, Please do not run it again!"),
                    w!("Tips"),
                    MB_OK,
                );
            }
            std::process::exit(0);
        }
    }
}

/// make sure app running at single-case
#[cfg(not(target_os = "windows"))]
pub fn make_sure_single_case() {
    //todo: unix system should be complete
}
/// init sqlite
pub fn init_sqlite() -> Result<()> {
    let data = path::app_config_dir(&Default::default());
    if let Some(data) = data {
        // app config dir
        let p = data.join("link.xsa.bs");
        // dir not exits
        if p.exists() == false {
            // create
            std::fs::DirBuilder::new().recursive(true).create(&p)?;
        }
        // check sqlite is init
        let inited = p.join(".inited");
        // if inited
        if inited.exists() == true {
            // return
            return Ok(());
        }
        // create sqlite database file
        let d = p.join(".data.db");
        // file not exists
        if d.exists() == false {
            // create
            std::fs::File::create(&d)?;
        }
        // to string
        let filepath = d.display().to_string();
        // connect and init sqlite tables
        let sp: Result<()> = tokio::runtime::Runtime::new()?.block_on(async move {
            init_sqlite_tables(&filepath).await?;
            Ok(())
        });
        // do error logic
        return match sp {
            Ok(_) => {
                // save inited status
                std::fs::File::create(&inited)?;
                Ok(())
            }
            Err(e) => {
                Err(anyhow::anyhow!("{}",e))
            }
        };
    }
    Ok(())
}

/// get sqlite connection instance
pub async fn get_connection() -> Result<SqliteConnection> {
    let data = path::app_config_dir(&Default::default());
    if let Some(data) = data {
        // app config dir
        let p = data.join("link.xsa.bs");
        // get sqlite database file
        let d = p.join(".data.db");
        Ok(SqliteConnection::connect(&d.display().to_string()).await?)
    } else {
        Err(anyhow!("get database path fail!"))
    }
}

/// 初始化sqlite表
async fn init_sqlite_tables(db_path: &str) -> Result<()> {
    let mut conn = SqliteConnection::connect(db_path).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS connections (
                id              TEXT PRIMARY KEY,
                name            TEXT NOT NULL,
                address              TEXT NOT NULL,
                port            TEXT DEFAULT '6379',
                username        TEXT DEFAULT '',
                password             TEXT DEFAULT ''
        )"
    )
        .execute(&mut conn)
        .await?;
    Ok(())
}

/// extract str to type<T>
pub fn extract<T: serde::de::DeserializeOwned>(data: &str) -> Result<T> {
    Ok(serde_json::from_str::<T>(data)?)
}