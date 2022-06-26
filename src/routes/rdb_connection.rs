use crate::rdb::{active_client, ActiveClient};
use crate::response::{Body, Response};
use crate::sqlite::ConnectionsTable;
use anyhow::Result;
use redis::{cmd, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub async fn change_active_client(data: &str) -> Result<impl Body> {
    let data: ConnectionsTable = serde_json::from_str(data)?;
    let inf = active_client()
        .lock()
        .set_port(data.port.parse()?)
        .set_ip(data.ip)
        .set_username(data.username)
        .set_password(data.password)
        .set_name(data.name)
        .generate();
    let _ = Client::open(inf)?.get_tokio_connection().await?;
    Ok(true)
}

pub async fn change_active_client_db(data: &str) -> Result<impl Body> {
    let inf = active_client().lock().set_db(data.parse()?).generate();
    let _ = Client::open(inf)?.get_tokio_connection().await?;
    Ok(true)
}

pub async fn info(_section: &str) -> Result<impl Body> {
    let inf = active_client().lock().generate();
    let client = Client::open(inf)?;
    let mut conn = client.get_tokio_connection().await?;
    let res: String = if _section == "*" {
        cmd("info").query_async(&mut conn).await?
    } else {
        cmd("info").arg(_section).query_async(&mut conn).await?
    };

    #[derive(Deserialize, Serialize)]
    struct Data {
        pub conn_info: ActiveClient,
        pub info: String,
    }
    Ok(Response::ok(
        Data {
            conn_info: active_client().lock().generate(),
            info: res,
        },
        None,
    ))
}
pub async fn cfg(section: &str) -> Result<impl Body> {
    let inf = active_client().lock().generate();
    let mut conn = Client::open(inf)?.get_tokio_connection().await?;
    let data: Vec<String> = cmd("config")
        .arg("get")
        .arg(section)
        .query_async(&mut conn)
        .await?;
    let mut map = HashMap::new();
    for (k, v) in data.iter().enumerate() {
        if k % 2 == 0 {
            map.insert(v.clone(), data[k + 1].clone());
        };
    }
    Ok(map)
}
