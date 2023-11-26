use crate::{
    models::Connections,
    response::{Body, Response, ResponseResult},
    utils::extract,
};
use log::debug;
use redis::{cmd, Commands, ErrorKind, RedisError};
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
    let key2 = params.key.clone();
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
        .do_query();
    let value = match value {
        Ok(value) => value,
        Err(err) => {
            match err.downcast_ref::<RedisError>() {
                Some(err) => match err.kind() {
                    ErrorKind::TypeError => {
                        if err.to_string().contains("response was nil") {
                            return Response::fail(
                                "",
                                Some(&format!(
                                    "Key '{:?}' not exists.",
                                    key2.unwrap_or("".to_owned())
                                )),
                            )
                            .into_response();
                        }
                    }
                    _ => {
                        debug!("Redis Kind _ {}", err);
                    }
                },
                None => {
                    debug!("Redis Error None,{}", err);
                }
            };
            return Response::fail("", None).into_response();
        }
    };
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
#[derive(Debug, Deserialize, Serialize, Clone)]
struct KeysParams {
    pub id: String,
    pub db: Option<i32>,
    pub cursor: Option<i32>,
    pub pattern: Option<String>,
    pub count: Option<i32>,
}
pub async fn keys(payload: &str) -> ResponseResult {
    let params = extract::<KeysParams>(payload)?;
    let params2 = params.clone();
    let mut clients = Connections::get(&params.id)
        .await?
        .connect::<(String, Vec<String>)>(params.db)
        .await?;

    clients
        .cluster_query(move |conn| {
            let res = cmd("scan")
                .arg(&params.cursor)
                .arg("match")
                .arg(&params.pattern)
                .arg("count")
                .arg(&params.count)
                .query(conn)?;

            Ok(res)
        })
        .single_query(move |conn| {
            let res = cmd("scan")
                .arg(&params2.cursor)
                .arg("match")
                .arg(&params2.pattern)
                .arg("count")
                .arg(&params2.count)
                .query(conn)?;

            Ok(res)
        });
    let data = clients.do_query()?;
    debug!("data: {:?}", data);
    #[derive(Debug, Serialize)]
    struct Data {
        pub cursor: i32,
        pub keys: Vec<String>,
    }
    let cursor = data.0;
    let mut keys = vec![];
    for v in data.1 {
        keys.push(v);
    }
    let data = Data {
        cursor: cursor.parse()?,
        keys,
    };
    Response::ok(data, None).into_response()
}

pub async fn delete_key(payload: &str) -> ResponseResult {
    let data = extract::<Params>(payload)?;
    let key = data.key.clone();
    let mut client = Connections::get(&data.id)
        .await?
        .connect::<()>(data.db)
        .await?;

    client
        .cluster_query(move |conn| {
            conn.del(&data.key)?;
            Ok(())
        })
        .single_query(move |conn| {
            conn.del(&key)?;
            Ok(())
        });
    client.do_query()?;
    Response::ok("", None).into_response()
}
