use std::collections::HashMap;
use crate::app::get_params;
use crate::response::{Body, Response};
use crate::sqlite::create_connection;
use anyhow::{Result};
use futures::future::Either;
use rusqlite::params;

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
        "insert into connections(id,name,address,port,username,password) values(?1,?2,?3,?4,?5,?6)",
        params![&id,name,ip,port,username,password],
    )?;
    if res > 0 {
        let mut map = HashMap::new();
        map.insert("a", "b");
        map.insert("b", "b");
        map.insert("c", "b");
        Ok(Response::ok(Some(map), None))
    } else {
        Ok(Response::fail(None, Some("666")))
    }
}

pub async fn search(data: &str) -> Result<Either<Response<i32>, Response<&str>>> {
    if data == "a" {
        Ok(Either::Left(Response::new(310, 666, "")))
    } else {
        Ok(Either::Right(Response::new(310, "666", "")))
    }
}

