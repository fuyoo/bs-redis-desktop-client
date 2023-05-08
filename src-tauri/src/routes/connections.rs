use crate::{
    models::Connections,
    response::{Body, Response},
    utils::extract,
};

use crate::response::ResponseResult;
use log::debug;
use nanoid::nanoid;
use redis::Value;
use std::collections::HashMap;

pub async fn get_connections_list(_payload: &str) -> ResponseResult {
    let res = Connections::get_all().await?;
    Response::ok(res, None).into_response()
}

pub async fn update_connection(payload: &str) -> ResponseResult {
    debug!("{}", payload);
    let data = extract::<Connections>(payload)?;
    let id = data.id.clone();
    if data.insert_or_update().await? {
        Response::ok(id, None).into_response()
    } else {
        Response::fail("", Some("edit failed!")).into_response()
    }
}

pub async fn add_connection(payload: &str) -> ResponseResult {
    debug!("{:?}", payload);
    let mut data = extract::<Connections>(payload)?;
    let id = nanoid!();
    data.id = Some(id.clone());
    if data.insert_or_update().await? {
        Response::ok(id, None).into_response()
    } else {
        Response::fail("", Some("add failed!")).into_response()
    }
}

pub async fn delete_connection(payload: &str) -> ResponseResult {
    let data = extract::<HashMap<String, String>>(payload)?;
    if let Some(id) = data.get("id") && Connections::delete(&id).await? {
        Response::ok(true, None).into_response()
    } else {
        Response::fail(false, Some("record not exists")).into_response()
    }
}

pub async fn is_available(payload: &str) -> ResponseResult {
    debug!("{:?}", payload);
    let res = extract::<Connections>(payload)?
        .connect::<Value>(None)
        .await?
        .has_available_connection()?;
    Response::ok(res, Some("ok")).into_response()
}

pub async fn test_connection(payload: &str) -> ResponseResult {
    let res = extract::<Connections>(payload)?
        .connect::<Value>(None)
        .await?
        .has_available_connection()?;

    Response::ok(res, Some("ok")).into_response()
}
