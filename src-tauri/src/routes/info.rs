use crate::{
    models::Connections,
    response::{Body, Response, ResponseResult},
    utils::extract,
};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    pub id: String,
    pub db: Option<i32>,
    pub key: Option<String>,
}

pub async fn key(payload: &str) -> ResponseResult {
    debug!("payload: {}", payload);
    let params = extract::<Params>(payload)?;
    let key = params.key.clone();
    let value = Connections::get(&params.id)
        .await?
        .connect::<(String, i64, i64)>(params.db)
        .await?
        .cluster_query(move |mut conn| {
            let cluster_value = redis::pipe()
                .cmd("type")
                .arg(key.clone())
                .cmd("pttl")
                .arg(key.clone())
                .cmd("memory")
                .arg("usage")
                .arg(key.clone())
                .query(&mut conn)?;
            Ok(cluster_value)
        })
        .single_query(move |mut conn| {
            let cluster_value = redis::pipe()
                .cmd("type")
                .arg(&params.key)
                .cmd("pttl")
                .arg(&params.key)
                .cmd("memory")
                .arg("usage")
                .arg(&params.key)
                .query(&mut conn)?;
            Ok(cluster_value)
        })
        .do_query()?;

    #[derive(Debug, Serialize)]
    struct Val {
        #[serde(rename = "type")]
        pub _type: String,
        pub pttl: i64,
        pub memory: i64,
    }
    let data = Val {
        _type: value.0,
        pttl: value.1,
        memory: value.2,
    };

    Response::ok(data, None).into_response()
}

pub async fn database(payload: &str) -> ResponseResult {
    let params = extract::<Params>(payload)?;
    let mut clients = Connections::get(&params.id)
        .await?
        .connect::<((String, String), i64, String)>(params.db)
        .await?;
    clients
        .cluster_query(|mut conn| {
            let r = redis::pipe()
                .cmd("config")
                .arg("databases")
                .cmd("dbsize")
                .arg("")
                .query(&mut conn)?;
            Ok(r)
        })
        .single_query(|mut conn| {
            let r = redis::pipe()
                .cmd("config")
                .arg("get")
                .arg("databases")
                .cmd("dbsize")
                .cmd("info")
                .arg("memory")
                .query(&mut conn)?;
            Ok(r)
        });
    let res = clients.do_query()?;
    #[derive(Serialize)]
    struct Info {
        pub database: String,
        pub keys: i64,
        pub memory: String,
    }
    let info = Info {
        database: res.0 .1,
        keys: res.1,
        memory: res.2,
    };
    Response::ok(info, None).into_response()
}
