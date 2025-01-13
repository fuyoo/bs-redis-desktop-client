use std::future::Future;

use crate::api::rdb::{ConnectionImpl, RedisClientImpl};
use crate::api::resp::Response;
use redis::cmd;
use tauri::{command, Result};

pub mod rdb;
pub mod resp;
pub mod router;
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
        "/status" => route(status(connection_info)).await,
        "/info" => route(base_info(connection_info, data)).await,
        "/keys" => route(keys(connection_info, data)).await,
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
async fn base_info(
    connection_info: ConnectionImpl,
    data: &str,
) -> Result<Response<Option<String>>> {
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

// search keys
async fn keys(connection_info: ConnectionImpl, data: &str) -> Result<Response<Vec<String>>> {
    let client = connection_info.into_client()?;
    Ok(r_ok!(vec![], None))
}
