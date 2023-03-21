use crate::{
    models::Connections,
    response::{Body, Response},
    utils::{create_proxy, extract},
};
use anyhow::Result;
use nanoid::nanoid;
use std::collections::HashMap;

pub async fn get_connections_list(_payload: &str) -> Result<String> {
    Response::ok(Connections::get_all().await?, None).into_response()
}

pub async fn update_connection(payload: &str) -> Result<String> {
    println!("{}",payload);
    let data = extract::<Connections>(payload)?;
    let id = data.id.clone();
    if data.insert_or_update().await? {
        Response::ok(id, None).into_response()
    } else {
        Response::fail("", Some("edit failed!")).into_response()
    }
}

pub async fn add_connection(payload: &str) -> Result<String> {
    println!("{}",payload);
    let mut data = extract::<Connections>(payload)?;
    let id = nanoid!();
    data.id = Some(id.clone());
    if data.insert_or_update().await? {
        Response::ok(id, None).into_response()
    } else {
        Response::fail("", Some("add failed!")).into_response()
    }
}

pub async fn delete_connection(payload: &str) -> Result<String> {
    let data = extract::<HashMap<String, String>>(payload)?;
    if let Some(id) = data.get("id") && Connections::delete(&id).await? {
        Response::ok(true, None).into_response()
    } else {
        Response::fail(false, Some("record not exists")).into_response()
    }
}
