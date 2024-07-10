use redis::{Client, Cmd, FromRedisValue};
use serde::{Deserialize, Serialize};

// connection info
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: Option<String>,
    pub db: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

// connection impl
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionImpl {
    pub id: usize,
    pub name: String,
    pub node: Vec<ConnectionInfo>,
    pub cluster: Option<bool>,
}

impl ConnectionImpl {
    pub fn into_client(self) -> anyhow::Result<impl RedisClientImpl> {
        if self.cluster.is_none() {
            let cfg = &self.node[0];
            RedisSingleClient::connect(cfg)
        } else {
            // 这里等下一步去实现
            todo!()
        }
    }
}


pub trait RedisClientImpl {
     fn do_command<T: FromRedisValue>(self, cmd: &Cmd) -> anyhow::Result<T>;
}

pub struct RedisSingleClient(Client);

impl RedisClientImpl for  RedisSingleClient {
      fn do_command<T: FromRedisValue>(self, cmd: &Cmd) -> anyhow::Result<T> {
        let mut conn = self.0.get_connection()?;
        return Ok(cmd.query::<T>(&mut conn)?);
    }
}
impl RedisSingleClient {
    pub fn connect(cfg: &ConnectionInfo) -> anyhow::Result<RedisSingleClient> {
        let c = Client::open(format!(
            "redis://{}:{}@{}:{}/{}?timeout=3s",
            cfg.username.clone().unwrap_or("".to_string()),
            cfg.password.clone().unwrap_or("".to_string()),
            cfg.host,
            cfg.port.clone().unwrap_or("6379".to_string()),
            cfg.db.clone().unwrap_or("0".to_string())
        ))?;
        Ok(RedisSingleClient(c))
    }

}


