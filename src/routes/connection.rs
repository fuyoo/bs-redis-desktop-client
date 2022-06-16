use crate::response::{Body, Response};
use crate::sqlite::{ConnectionsTable, create_connection};
use anyhow::{Result};
use rusqlite::params;
use crate::rdb::{ActiveClient};

pub async fn create(data: &str) -> Result<impl Body> {
    let params: ConnectionsTable = serde_json::from_str(data)?;
    let id = uuid::Uuid::new_v4().to_string();
    let conn = create_connection().await?;
    let res = conn.execute(
        "insert into connections(id,name,ip,port,username,password) values(?1,?2,?3,?4,?5,?6)",
        params![id, params.name, params.ip, params.port, params.username, params.password],
    )?;
    if res > 0 {
        Ok(Response::ok(Some(true), None))
    } else {
        Ok(Response::fail(None::<bool>, Some("666")))
    }
}

pub async fn list() -> Result<Response<Vec<ConnectionsTable>>> {
    let conn = create_connection().await?;
    let mut stmt = conn.prepare("select * from connections")?;
    let rows = stmt.query_map([], |row| {
        Ok(ConnectionsTable {
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

pub async fn delete(id: &str) -> Result<Response<bool>> {
    let conn = create_connection().await?;
    if conn.execute("delete from connections where id = ?1", params![&id])? > 0 {
        Ok(Response::ok(true, None))
    } else {
        Ok(Response::fail(false, None))
    }
}

pub async fn update(data: &str) -> Result<Response<bool>> {
    let params: ConnectionsTable = serde_json::from_str(data)?;
    let conn = create_connection().await?;
    if conn.execute(
        "update connections set name=?1,ip=?2,port=?3,username=?4,password=?5 where id = ?6",
        params![params.name,params.ip,params.port,params.username,params.password,params.id])? > 0 {
        Ok(Response::ok(true, None))
    } else {
        Ok(Response::fail(false, None))
    }
}

pub async fn test_connection(data: &str) -> Result<Response<bool>> {
    let conninfo: ConnectionsTable = serde_json::from_str(data)?;
    let inf = ActiveClient::default().set_ip(conninfo.ip)
        .set_password(conninfo.password)
        .set_port(conninfo.port.parse::<usize>()?)
        .set_username(conninfo.username);
    let client = redis::Client::open(inf)?;
    let _ = client.get_tokio_connection().await?;
    Ok(Response::ok(true, Some("连接成功")))
}