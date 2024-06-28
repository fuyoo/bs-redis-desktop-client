use log::error;
use serde::Serialize;
use tauri::command;

pub trait IntoResponse {
    fn into_response(self) -> String;
}

#[derive(Serialize, Debug)]
pub struct Response {}

impl Response {
    pub fn into_response(self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|e| {
            error!("{:?}", e);
            String::from("{}")
        })
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
pub fn request(rid: &str, action: &str, connection_info: &str, data: &str) -> String {
    println!("rid: {}\naction: {}\nconnection_info: {}\ndata: {}", rid,action,connection_info, data);

    let resp = Response {};
    resp.into_response()
}
