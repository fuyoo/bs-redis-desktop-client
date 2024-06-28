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
pub fn request(rid: &str, action: &str, connectionInfo: &str, data: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", action);

    let resp = Response {};
    resp.into_response()
}
