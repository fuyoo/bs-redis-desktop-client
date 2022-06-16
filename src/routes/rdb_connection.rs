use crate::response::Body;
use anyhow::Result;
use log::info;
use redis::{Client, cmd};
use crate::rdb::ActiveClient;

pub async fn change_active_client() -> Result<impl Body> {
    Ok("".to_string())
}

pub async fn info(_section: &str) -> Result<impl Body> {
    let inf = ActiveClient::default().set_ip("127.0.0.1".to_string())
        .set_port(6379);
    let client = Client::open(inf)?;
    let mut conn = client.get_tokio_connection().await?;
    let res: String = if _section == "*" {
        cmd("info").query_async(&mut conn).await?
    } else {
        cmd("info").arg(_section).query_async(&mut conn).await?
    };
    info!("{:?}",res);
    Ok(res)
}