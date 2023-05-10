use crate::{
    models::Connections,
    response::{Body, Response, ResponseResult},
    utils::extract,
};
use log::debug;
use redis::Commands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize,Serialize)]
pub struct KeyParams {
    pub id: String,
    pub db: Option<i32>,
    pub k: String,
}
pub async fn get(payload: &str) -> ResponseResult {
    debug!("{:?}", payload);
    let key_params = extract::<KeyParams>(payload)?;
    let connection = Connections::get(&key_params.id).await?;
    let mut conn = connection.connect::<String>(key_params.db).await?;
    let key = key_params.k.clone();
    conn.cluster_query(move |conn| {
        let data = conn.get(&key)?;
        Ok(data)
    })
    .single_query(move |conn| {
        let data = conn.get(&key_params.k)?;
        Ok(data)
    });
    let data = conn.do_query()?;
    Response::ok(data, None).into_response()
}
