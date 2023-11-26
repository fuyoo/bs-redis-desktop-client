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

use crate::models::Connections;
use log::debug;
use redis::{
    cluster::{ClusterClient, ClusterConnection},
    Client,
};
use tauri::api::path;
use tokio::{sync::oneshot, task::JoinHandle as TokioJoinHandle};
#[cfg(target_os = "windows")]
use windows::{
    Win32::{
        Foundation::{GetLastError, WIN32_ERROR,HWND},
        Globalization::GetUserDefaultUILanguage,
        System::Threading::{CreateMutexW, OpenMutexW, SYNCHRONIZATION_ACCESS_RIGHTS},
        UI::WindowsAndMessaging::{MessageBoxW, MB_OK,FindWindowW, SetForegroundWindow, ShowWindow, SW_NORMAL},
    },
    core::w,
};

/// make sure app running at single-case
#[cfg(target_os = "windows")]
pub fn make_sure_single_case() {
    unsafe {
        let  hwnd  = FindWindowW(None,w!("BS"));

        if hwnd != HWND(0) {
            std::process::exit(0);
        } else {
            ShowWindow(hwnd,SW_NORMAL);
            SetForegroundWindow(hwnd);
        }
       /* let _ = OpenMutexW(
            SYNCHRONIZATION_ACCESS_RIGHTS(0),
            true,
            w!("link.xsa.bs@fuyoo"),
        );
        let res = GetLastError();
        match res {
            Err(_) => {
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
            Ok(_) => {
                // generate lock
                let _ = CreateMutexW(None, true, w!("link.xsa.bs@fuyoo"));
            }
        }*/

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
        debug!("sqlite pat");
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

/// get the idle port
pub fn get_idle_port() -> Result<u16> {
    // input a port 0,mean's tell the kernel give us an idle port!;
    Ok(TcpListener::bind(("127.0.0.1", 0))?.local_addr()?.port())
}

pub trait DoQuery<T> {
    fn query_cluster(&mut self, f: impl Fn(&mut ClusterConnection) -> Result<T>);
    fn query_single(&mut self, f: impl Fn(&mut redis::Connection) -> Result<T>);
}

pub struct RdbClients<T> {
    cluster: Option<ClusterClient>,
    single: Option<Client>,
    cluster_fn: Option<Box<dyn Fn(&mut ClusterConnection) -> Result<T>>>,
    single_fn: Option<Box<dyn Fn(&mut redis::Connection) -> Result<T>>>,
}

impl<T> RdbClients<T> {
    pub fn new(params: &Connections, database: Option<i32>) -> Result<Self> {
        // check is the cluster connection
        let is_cluster = if let Some(cluster) = params.cluster {
            cluster
        } else {
            false
        };
        let mut rdb_clients = RdbClients {
            cluster: None,
            single: None,
            cluster_fn: None,
            single_fn: None,
        };
        // is cluster
        if is_cluster {
            let c = create_cluster_redis_client(params)?;
            rdb_clients.cluster = Some(c);
        } else {
            let c = create_single_connection(&params, database)?;
            rdb_clients.single = Some(c)
        }
        Ok(rdb_clients)
    }
    pub fn single_query(
        &mut self,
        f: impl Fn(&mut redis::Connection) -> Result<T> + 'static,
    ) -> &mut RdbClients<T> {
        self.single_fn = Some(Box::new(f));
        self
    }
    pub fn cluster_query(
        &mut self,
        f: impl Fn(&mut ClusterConnection) -> Result<T> + 'static,
    ) -> &mut RdbClients<T> {
        self.cluster_fn = Some(Box::new(f));
        self
    }
    pub fn has_available_connection(&self) -> Result<bool> {
        if let Some(cluster) = &self.cluster {
            let _ = cluster.get_connection()?;
            return Ok(true);
        }
        if let Some(single) = &self.single {
            let _ = single.get_connection()?;
            return Ok(true);
        }
        Err(anyhow!("No connection available!"))
    }
    pub fn do_query(&mut self) -> Result<T> {
        if let Some(cluster) = &self.cluster {
            return if let Some(f) = &self.cluster_fn {
                let mut conn = cluster.get_connection()?;
                f(&mut conn)
            } else {
                Err(anyhow!("not a cluster connection!"))
            };
        }

        if let Some(single) = &self.single {
            return if let Some(f) = &self.single_fn {
                let mut conn = single.get_connection()?;
                f(&mut conn)
            } else {
                Err(anyhow!("not a single connection!"))
            };
        }

        Err(anyhow!("cannot find any available connections!"))
    }
}

pub fn create_single_connection(params: &Connections, database: Option<i32>) -> Result<Client> {
    // generate uri
    let uri = format!(
        r#"redis://{}:{}@{}:{}/{}?timeout=10s"#,
        &params.username.clone().unwrap_or("".to_string()),
        &params.password.clone().unwrap_or("".to_string()),
        &params.address.clone(),
        &params.port.clone().unwrap_or(6379),
        &database.clone().unwrap_or(0)
    );
    debug!("connection uri: {:?}", uri);
    // crate a client
    let client = Client::open(uri)?;
    // return
    Ok(client)
}

pub fn create_cluster_redis_client(params: &Connections) -> Result<ClusterClient> {
    let nodes = params
        .nodes
        .clone()
        .unwrap_or("".to_string())
        .split(",")
        .enumerate()
        .map(|x| x.1.to_owned())
        .collect::<Vec<String>>();
    debug!("cluster nodes: {:?}", nodes);
    let client = ClusterClient::new(nodes)?;
    Ok(client)
}
