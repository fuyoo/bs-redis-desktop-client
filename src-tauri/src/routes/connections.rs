use anyhow::Result;
use crate::models::Connections;
use crate::response::{Body, Response};
use crate::utils::{extract, get_connection};

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

pub async fn add_connection(payload: &str) -> Result<String> {
    let mut conn = get_connection().await?;
    let data = extract::<Connections>(payload)?;
    println!("{:#?}",data);
    /*let res = sqlx::query(r"
        insert into connections(
        id,
        name,
        address,
        port,
        username,
        password)
        values (?1,?2,?3,?4,?5,?6)
    ");
       // .bind()*/
    Response::ok("",None).into_response()

}