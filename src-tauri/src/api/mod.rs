use redis::cmd;
use tauri::{command, Result};
use crate::api::resp::{IntoResponse};
use crate::api::rdb::RedisClientImpl;
use crate::api::resp::Response;

pub mod rdb;
pub mod resp;
use crate::{r_404, r_ok};

#[command]
pub async fn request(path: &str, connection_info: rdb::ConnectionImpl, data: &str)
                     -> Result<String> {
    match path {
        "/status" => status(connection_info, data).await?.into_response(),
        &_ => r_404!(path).into_response()
    }
}

//
async fn status(connection_info: rdb::ConnectionImpl, data: &str) -> Result<Response<Option<String>>> {
    let r = connection_info.into_client()?.do_command::<Option<String>>(&cmd("ping")).await?;
    Ok(r_ok!(r,None))
}



