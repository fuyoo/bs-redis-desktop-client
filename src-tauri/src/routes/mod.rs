pub mod connections;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use tauri::command;

use crate::response::{Body, Response};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub path: String,
    pub payload: String,
}

// dispatch actions
async fn dispatch(path: &str, payload: &str) -> Result<String> {
    // register routes
    match path {
        "/connection/list" => connections::get_connections_list(&payload).await,
        "/connection/add" => connections::add_connection(&payload).await,
        "/connection/delete" => connections::delete_connection(&payload).await,
        "/connection/edit" => connections::update_connection(&payload).await,
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
            let res = r#""code":500,data:null,msg:""#;
            Ok(format!("{}{}{}{}", "{", res, err.to_string(), "\"}"))
        }
    };
}
