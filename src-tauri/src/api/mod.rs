pub mod rdb;
pub mod resp;

use redis::cmd;
use tauri::command;
use resp::Response;
use crate::r_ok;
use crate::api::rdb::RedisClientImpl;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
pub async fn request(rid: &str, action: &str, connection_info: rdb::ConnectionImpl, data: &str) -> tauri::Result<Response<Option<String>>> {
    println!("rid: {}\naction: {}\nconnection_info: {:?}\ndata: {:?}", rid, action, connection_info, data);
    let r = connection_info.into_client().unwrap().do_command::<Option<String>>(&cmd("get").arg("123"))?;
    println!("{:#?}", r);
    Ok(r_ok!(r, None))
}



