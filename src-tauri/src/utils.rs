use anyhow::{anyhow, Result};
use std::{
    borrow::Cow,
    net::{IpAddr, Ipv4Addr},
};

use crate::structs::{AuthType, SshJumpTaskInfo};
use sqlx::{Connection, SqliteConnection};
use ssh_jumper::{
    model::{HostAddress, HostSocketParams, JumpHostAuthParams, SshTunnelParams},
    SshJumper,
};
use std::{net::TcpListener, path::PathBuf};
use tauri::api::path;
use tokio::{sync::oneshot, task::JoinHandle as TokioJoinHandle};
#[cfg(target_os = "windows")]
use windows::{
    w,
    Win32::{
        Foundation::{GetLastError, WIN32_ERROR},
        Globalization::GetUserDefaultUILanguage,
        System::Threading::{CreateMutexW, OpenMutexW, SYNCHRONIZATION_ACCESS_RIGHTS},
        UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
    },
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
            Err(e) => Err(anyhow::anyhow!("{}", e)),
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

/// init sqlite tables
async fn init_sqlite_tables(db_path: &str) -> Result<()> {
    let mut conn = SqliteConnection::connect(db_path).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS connections (
                id              TEXT    primary key not null,
                name            TEXT    not null,
                address         TEXT    default '',
                port            INTEGER default 6379,
                username        TEXT    default '',
                password        TEXT    default '',
                cluster         INT     default 0,
                nodes           TEXT    default '',
                proxy           INT     default 0,
                proxy_key_type  INT     default 0,
                proxy_username  TEXT    default '',
                proxy_address   TEXT    default '',
                proxy_port      INTEGER default 6379,
                proxy_password  TEXT    default '',
                proxy_file_path TEXT    default ''
        )",
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

/// extract str to type<T>
pub fn extract<T: serde::de::DeserializeOwned>(data: &str) -> Result<T> {
    Ok(serde_json::from_str::<T>(data)?)
}
/// create proxy
pub fn create_proxy(info: SshJumpTaskInfo) -> Result<(oneshot::Receiver<bool>, u16)> {
    let (tx, tr) = oneshot::channel();
    let local_port = get_idle_port()?;
    let _: TokioJoinHandle<Result<()>> = tokio::spawn(async move {
        // parse address
        let addr = info
            .host
            .parse::<IpAddr>()
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        let jump_host = HostAddress::IpAddr(addr);
        // generate jump host
        let res = match info.auth {
            AuthType::Password(pwd) => Some(JumpHostAuthParams::password(
                Cow::from(info.user),
                Cow::from(pwd),
            )),
            AuthType::PublicKey(key_path) => Some(JumpHostAuthParams::key_pair(
                Cow::from(info.user),
                Cow::from(PathBuf::from(key_path)),
                None,
            )),
            AuthType::Unknown => None,
        };
        // condition
        if let Some(res) = res {
            let target_socket = HostSocketParams {
                address: jump_host.clone(),
                port: info.port as u16,
            };
            // start proxy
            let ssh_params = SshTunnelParams::new(jump_host, res, target_socket)
                // Optional: OS will allocate a port if this is left out
                .with_local_port(local_port);
            let (_, r) = SshJumper::open_tunnel(&ssh_params).await?;
            // we not care this result, ignore it!
            let _ = tx.send(true);
            // listen
            let _ = r.await;
        } else {
            // we not care this result, ignore it!
            let _ = tx.send(false);
        };
        Ok(())
    });

    Ok((tr, local_port))
}

pub fn get_idle_port() -> Result<u16> {
    // input a port 0,mean's tell the kernel give us an idle port!;
    Ok(TcpListener::bind(("127.0.0.1", 0))?.local_addr()?.port())
}
