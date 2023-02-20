use std::collections::HashMap;
use anyhow::Result;
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

impl Body for String {
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}

impl<T> Body for Response<T>
    where
        T: serde::ser::Serialize,
{
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

impl Body for () {
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok("", None))?)
    }
}

impl Body for bool {
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}

impl<T> Response<T>
    where
        T: serde::ser::Serialize,
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

impl<T> Body for Vec<T>
    where
        T: serde::ser::Serialize,
{
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}

impl<K, V> Body for HashMap<K, V>
    where
        K: serde::ser::Serialize + Eq + std::hash::Hash,
        V: serde::ser::Serialize + Eq + std::hash::Hash,
{
    fn into_response(self) -> Result<String> {
        Ok(serde_json::to_string(&Response::ok(self, None))?)
    }
}