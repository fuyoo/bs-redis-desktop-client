use anyhow::Result;
use crate::models::Connections;
use crate::response::{Body, Response};
use crate::utils::{get_connection};

pub async fn get_connections_list(_payload: &str) -> Result<String> {
    let mut conn = get_connection().await?;
    let res = sqlx::query("select * from connections")
        .fetch_all(&mut conn)
        .await?.iter().map(|row| {
        Connections::bind(row)
    }).collect::<Vec<Connections>>();

    println!("{:#?}", res);
    Response::ok(res, None).into_response()
}