use log::debug;
use redis::{cmd, Commands, Value};
use serde::{Deserialize, Serialize};
use crate::models::Connections;
use crate::response::{Body, Response, ResponseResult};
use crate::utils::{extract};

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    pub id: String,
    pub db: Option<i32>,
    pub key: Option<String>,
}

pub async fn key(payload: &str) -> ResponseResult {
    debug!("payload: {}", payload);
    let params = extract::<Params>(payload)?;
    let mut conn = Connections::get(&params.id).await?.connect(params.db).await?;
    let value = conn.run::<Value>(|mut conn| {
        let res = cmd("get")
            .query::<Value>(&mut conn)?;
        Ok(res)
    }, |mut conn| {
        let res = cmd("get")
            .query::<Value>(&mut conn)?;
        Ok(res)
    })?;


    #[derive(Debug, Serialize)]
    struct Val {
        #[serde(rename = "type")]
        pub _type: String,
        pub pttl: i64,
        pub memory: i64,
    }
    let mut data = Val {
        _type: "".to_string(),
        pttl: 0,
        memory: 0,
    };
    match value {
        Value::Bulk(val) => {
            for (i, val) in val.iter().enumerate() {
                match val {
                    Value::Int(val) => {
                        if i == 1 {
                            data.pttl = val.to_owned()
                        } else {
                            data.memory = val.to_owned()
                        }
                    }
                    Value::Status(val) => {
                        data._type = val.to_owned()
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Response::ok(data, None).into_response()
}

async fn database(payload: &str) -> ResponseResult {
    let params = extract::<Params>(payload)?;
    let res = Connections::get(&params.id)
        .await?
        .connect(None)
        .await?
        .run::<String>(|conn| {
            conn.get("")?
        }, |conn| {
            conn.get("")?
        })?;

    Ok("".to_string())
}