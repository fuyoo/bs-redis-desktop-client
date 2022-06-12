use crate::app::get_params;
use crate::response::{Body, Response};
use crate::sqlite::create_connection;
use anyhow::Result;
use futures::future::Either;
use rusqlite::params;
use serde::{Deserialize, Serialize};
pub async fn create(data: &str) -> Result<impl Body> {
    let params: serde_json::Value = serde_json::from_str(data)?;
    let name = get_params("name", true, &params)?;
    let ip = get_params("ip", true, &params)?;
    let port = get_params("port", true, &params)?;
    let username = get_params("username", false, &params)?;
    let password = get_params("password", false, &params)?;
    let id = uuid::Uuid::new_v4().to_string();
    let conn = create_connection().await?;
    let res = conn.execute(
        "insert into connections(id,name,ip,port,username,password) values(?1,?2,?3,?4,?5,?6)",
        params![&id, name, ip, port, username, password],
    )?;
    if res > 0 {
        Ok(Response::ok(Some(true), None))
    } else {
        Ok(Response::fail(None::<bool>, Some("666")))
    }
}
#[derive(Serialize, Deserialize)]
pub struct Row {
    id: String,
    name: String,
    ip: String,
    port: String,
    username: String,
    password: String,
}
pub async fn list() -> Result<Response<Vec<Row>>> {
    let conn = create_connection().await?;
    let mut stmt = conn.prepare("select * from connections")?;
    let rows = stmt.query_map([], |row| {
        Ok(Row {
            id: row.get("id")?,
            name: row.get("name")?,
            ip: row.get("ip")?,
            port: row.get("port")?,
            username: row.get("username")?,
            password: row.get("password")?,
        })
    })?;
    let mut datas = vec![];
    for item in rows {
        datas.push(item?);
    }
    Ok(Response::new(200, datas, ""))
}
