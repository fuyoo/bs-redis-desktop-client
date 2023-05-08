pub mod connections;
pub mod info;
pub mod string;

use anyhow::Result;
use log::error;
use serde::{Deserialize, Serialize};

use tauri::command;

use crate::response::{Body, Response, ResponseResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub path: String,
    pub payload: String,
}

// dispatch actions
async fn dispatch(path: &str, payload: &str) -> ResponseResult {
    // register routes
    match path {
        "/connection/list" => connections::get_connections_list(&payload).await,
        "/connection/add" => connections::add_connection(&payload).await,
        "/connection/delete" => connections::delete_connection(&payload).await,
        "/connection/edit" => connections::update_connection(&payload).await,
        "/connection/test" => connections::test_connection(&payload).await,
        "/connection/available" => connections::is_available(&payload).await,
        "/info/database" => info::database(&payload).await,
        "/info/keys" => info::keys(&payload).await,
        "/info/key" => info::key(&payload).await,
        "/key/string/get" => string::get(&payload).await,

        _ => Response::<Option<bool>>::new(
            404,
            None,
            format!("request path {} not found!", path).as_str(),
        )
            .into_response(),
    }
}

#[command]
pub async fn routes(path: String, payload: String) -> Result<String, String> {
    return match dispatch(&path, &payload).await {
        Ok(res) => Ok(res),
        Err(err) => {
            #[derive(Debug, Serialize)]
            struct ErrMsg {
                code: i32,
                data: Option<bool>,
                msg: String,
            }
            error!("{:?}", err.to_string());
            let res = serde_json::to_string(&ErrMsg { code: 500, data: None::<bool>, msg: err.to_string() })
                .unwrap_or(r#"{"code":500,"data":null,"msg":"system error！"}"#.to_owned());

            Ok(res)
        }
    };
}

#[command]
pub async fn exit() {
    std::process::exit(0)
}
