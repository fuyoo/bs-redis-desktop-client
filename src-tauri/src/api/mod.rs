use std::future::Future;

use super::tools::extract;
use crate::api::rdb::{ConnectionImpl, RedisClientImpl};
use crate::api::resp::Response;
use redis::{cmd, Value};
use serde::{Deserialize, Serialize};
use tauri::{command, Result};
pub mod rdb;
pub mod resp;
use crate::{r_404, r_error, r_ok};

async fn route<T: serde::Serialize>(f: impl Future<Output = Result<Response<T>>>) -> String {
    match f.await {
        Ok(r) => r.to_string(),
        Err(e) => r_error!(e).to_string(),
    }
}

#[command]
pub async fn request(
    path: &str,
    connection_info: rdb::ConnectionImpl,
    data: &str,
) -> Result<String> {
    let r = match path {
        "/cmd" => route(do_query(connection_info, data)).await,
        // checking connection status
        "/status" => route(status(connection_info)).await,
        // fetch redis info
        "/info" => route(info(connection_info, data)).await,
        &_ => r_404!(path).to_string(),
    };
    Ok(r)
}

// check connection status
async fn status(connection_info: ConnectionImpl) -> Result<Response<Option<String>>> {
    let r = connection_info
        .into_client()?
        .do_command::<Option<String>>(&cmd("ping"))
        .await?;
    Ok(r_ok!(r, None))
}

// base_info
async fn info(connection_info: ConnectionImpl, data: &str) -> Result<Response<Option<String>>> {
    if data == "" {
        let r = connection_info
            .into_client()?
            .do_command::<Option<String>>(&cmd("info"))
            .await?;
        return Ok(r_ok!(r, None));
    }
    let r = connection_info
        .into_client()?
        .do_command::<Option<String>>(&cmd("info").arg(data))
        .await?;
    Ok(r_ok!(r, None))
}

/// secarch keys param.
#[derive(Serialize, Deserialize, Debug)]
struct KeyParam {
    pub cursor: Option<usize>,
    pub key: String,
    pub count: Option<usize>,
}

// here,we provide a query function,to do all query from frontend.
async fn do_query(connection_info: ConnectionImpl, data: &str) -> Result<Response<String>> {
    let param = extract::<Vec<String>>(data)?;
    // just print log in debug environment
    if cfg!(debug_assertions) {
        println!("cmd param: {:#?}", param);
    }
    let mut cmd_ = cmd(param.get(0).unwrap());
    for (_, v) in param.iter().skip(1).enumerate() {
        cmd_.arg(v);
    }
    let client = connection_info.into_client()?;
    let resp = client.do_command::<Value>(&mut cmd_).await?;
    Ok(r_ok!(rdb::convert_to_string(resp)?, None))
}
