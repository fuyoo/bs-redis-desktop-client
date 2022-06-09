use anyhow::Result;
use futures::future::Either;
use serde::Serialize;

pub trait Body {
    fn into_response(self) -> Result<String>;
}

/// generate the unified response data
#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub code: usize,
    pub data: T,
    pub msg: String,
}

impl<A, B> Body for Either<A, B>
    where A: serde::ser::Serialize,
          B: serde::ser::Serialize,
{
    fn into_response(self) -> Result<String> {
        match self {
            Either::Left(data) => {
                Ok(serde_json::to_string(&data)?)
            }
            Either::Right(data) => {
                Ok(serde_json::to_string(&data)?)
            }
        }
    }
}

impl<T> Body for Response<T>
    where T: serde::ser::Serialize, {
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

impl Body for () {
    fn into_response(self) -> Result<String> {
        Ok("".to_string())
    }
}

impl<T> Response<T>
    where T: serde::ser::Serialize,
{
    pub fn new(code: usize, data: T, msg: &str) -> Self {
        Response {
            code,
            data,
            msg: msg.to_string(),
        }
    }

    pub fn ok(data: T, msg: Option<&str>) -> Self {
        Response {
            code: 200,
            data,
            msg: msg.unwrap_or("操作成功").to_string(),
        }
    }

    pub fn fail(data: T, msg: Option<&str>) -> Self {
        Response {
            code: 300,
            data,
            msg: msg.unwrap_or("操作失败").to_string(),
        }
    }
}

