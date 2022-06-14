use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use parking_lot::Mutex;
use redis::{ConnectionAddr, ConnectionInfo, IntoConnectionInfo, RedisResult};

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
}

impl ActiveClient {
    pub fn set_ip(mut self, ip: String) -> Self {
        self.ip = ip;
        self
    }
    pub fn set_port(mut self, port: usize) -> Self{
        self.port = port;
        self
    }
    pub fn set_db(mut self, port: i64) -> Self {
        self.db = port;
        self
    }
    pub fn set_username(mut self, username: String) -> Self{
        self.username = username;
        self
    }
    pub fn set_password(mut self, password: String) -> Self {
        self.password = password;
        self
    }
}

impl IntoConnectionInfo for ActiveClient {
    fn into_connection_info(self) -> RedisResult<ConnectionInfo> {
        let inf = ConnectionInfo {
            addr: Box::new(ConnectionAddr::Tcp(self.ip, self.port as u16)),
            db: self.db as i64,
            username: if self.username == "" {
                None
            } else {
                Some(self.username)
            },
            passwd: if self.password == "" {
                None
            } else {
                Some(self.password)
            },
        };
        Ok(inf)
    }
}