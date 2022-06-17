use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use redis::{ConnectionAddr, ConnectionInfo, IntoConnectionInfo, RedisResult};
use serde::{Deserialize, Serialize};

pub fn active_client() -> &'static Mutex<ActiveClient> {
    static INSTANCE: OnceCell<Mutex<ActiveClient>> = OnceCell::new();
    INSTANCE.get_or_init(|| Mutex::new(ActiveClient::default()))
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ActiveClient {
    pub ip: String,
    pub port: usize,
    pub db: i64,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl ActiveClient {
    pub fn set_ip(&mut self, ip: String) -> &mut Self {
        self.ip = ip;
        self
    }
    pub fn set_port(&mut self, port: usize) -> &mut Self {
        self.port = port;
        self
    }
    pub fn set_db(&mut self, port: i64) -> &mut Self {
        self.db = port;
        self
    }
    pub fn set_username(&mut self, username: String) -> &mut Self {
        self.username = username;
        self
    }
    pub fn set_password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn generate(&self) -> ActiveClient {
        ActiveClient {
            ip: self.ip.clone(),
            port: self.port,
            db: self.db,
            username: self.username.clone(),
            password: self.password.clone(),
            name: self.name.clone(),
        }
    }
}

impl IntoConnectionInfo for ActiveClient {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        let inf = ConnectionInfo {
            addr: Box::new(ConnectionAddr::Tcp(self.ip.clone(), self.port as u16)),
            db: self.db as i64,
            username: if self.username == "" {
                None
            } else {
                Some(self.username.to_string())
            },
            passwd: if self.password == "" {
                None
            } else {
                Some(self.password.to_string())
            },
        };
        Ok(inf)
    }
}
