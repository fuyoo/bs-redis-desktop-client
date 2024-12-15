use redis::cmd;
use tauri::{command, Result};
use crate::api::resp::IntoResponse;
use crate::api::rdb::{ConnectionImpl, RedisClientImpl};
use crate::api::resp::Response;

pub mod rdb;
pub mod resp;
use crate::{r_404, r_ok};

#[command]
pub async fn request(path: &str, connection_info: rdb::ConnectionImpl, data: &str)
                     -> Result<String> {
    match path {
        "/status" => status(connection_info).await?.into_response(),
        "/info" => base_info(connection_info,data).await?.into_response(),
        "/keys"=> keys(connection_info,data).await?.into_response(),
        &_ => r_404!(path).into_response()
    }
}

// check connection status
async fn status(connection_info: ConnectionImpl) -> Result<Response<Option<String>>> {
    let r = connection_info.into_client()?.do_command::<Option<String>>(&cmd("ping")).await?;
    Ok(r_ok!(r,None))
}

// base_info
async  fn base_info(connection_info: ConnectionImpl,data: &str) -> Result<Response<Option<String>>> {
    if data == "" {
        let r = connection_info.into_client()?.do_command::<Option<String>>(&cmd("info")).await?;
        return  Ok(r_ok!(r,None));
    }
    let r = connection_info.into_client()?.do_command::<Option<String>>(&cmd("info").arg(data)).await?;
    Ok(r_ok!(r,None))
}

// search keys
async fn keys(connection_info: ConnectionImpl,data: &str) -> Result<Response<Vec<String>>> {
    let client = connection_info.into_client()?;
    Ok(r_ok!(vec![],None))
}

