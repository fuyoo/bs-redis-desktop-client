use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use redis::{Client, Cmd, FromRedisValue, Value};
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
    pub id: Option<usize>,
    pub name: String,
    pub node: Vec<ConnectionInfo>,
    pub cluster: Option<bool>,
}

impl ConnectionImpl {
    pub fn into_client(self) -> anyhow::Result<impl RedisClientImpl> {
        if self.cluster.unwrap_or(false) == false {
            let cfg = &self.node[0];
            RedisSingleClient::connect(cfg)
        } else {
            // next work step.
            anyhow::bail!("cluster mode not support yet")
        }
    }
}

#[async_trait]
pub trait RedisClientImpl {
    async fn do_command<T: FromRedisValue>(self, cmd: &Cmd) -> anyhow::Result<T>;
}

pub struct RedisSingleClient(Client);

#[async_trait]
impl RedisClientImpl for RedisSingleClient {
    async fn do_command<T: FromRedisValue>(self, cmd: &Cmd) -> anyhow::Result<T> {
        let mut conn = self.0.get_connection_with_timeout(Duration::from_secs(5))?;
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

pub fn convert_to_string(ori: Value) -> Result<String> {
    let v = match ori {
        Value::Nil => "".to_string(),
        Value::Int(i) => i.to_string(),
        Value::BulkString(b) => {
            String::from_utf8_lossy(&b).into()
        }
        Value::Okay => "OK".to_string(),
        Value::SimpleString(s) => s,
        Value::Map(items) => {
            let mut result = String::new();
            for (i, (key, value)) in items.clone().into_iter().enumerate() {
                result.push_str(&format!(
                    "{}: {}",
                    convert_to_string(key)?,
                    convert_to_string(value)?
                ));
                if i != items.len() - 1 {
                    result.push_str(", ");
                }
            }
            result
        }
        Value::Attribute { data,.. } => convert_to_string(*data)?,
        Value::Set(values) => {
            let mut result = String::new();
            for (i, value) in values.clone().into_iter().enumerate() {
                result.push_str(&convert_to_string(value)?);
                if i != values.len() - 1 {
                    result.push_str(", ");
                }
            }
            result
        }
        Value::Double(d) => d.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::VerbatimString { format: _, text } => text,
        Value::BigNumber(big_int) => big_int.to_string(),
        Value::Push { kind: _, data } => {
            let mut result = String::new();
            for (i, item) in data.clone().into_iter().enumerate() {
                result.push_str(&convert_to_string(item)?);
                if i != data.len() - 1 {
                    result.push_str(", ");
                }
            }
            result
        }
        Value::ServerError(server_error) => server_error.to_string(),
        Value::Array(d) => {
            let mut s = "".to_string();
            for (k, v) in d.iter().enumerate() {
                s += &convert_to_string(v.clone())?;
                if k != d.len() - 1 {
                    s += "\n";
                }
            }
            s
        }
        _ => "".to_string(),
    };
    Ok(v)
}
