use crate::app::app;
use crate::sqlite::create_connection;
use anyhow::Error;
use log::info;
use rusqlite::params;
use sciter::{make_args, Value};

pub async fn create(data: String, cb: Value) -> Result<(), Error> {
    let sd: serde_json::Value = serde_json::from_str(&data)?;
    let id =uuid::Uuid::new_v4().to_string();
    let conn = create_connection().await?;
    let res = conn.execute(
        "insert into connections(id,name,address,port,username,password) values(?1,?2,?3,?4,?5,?6)",
        params![
            &id,
            sd["name"].as_str().unwrap_or(""),
            sd["address"].as_str().unwrap_or(""),
            sd["port"].as_str().unwrap_or(""),
            sd["username"].as_str().unwrap_or(""),
            sd["password"].as_str().unwrap_or("")],
    )?;

    if res > 0 {
        cb.call(None,&make_args!("{\"code\":200}"),None)?;
    } else {
        cb.call(None, &make_args!("{\"code\":300}"), None)?;
    }
    Ok(())
}

