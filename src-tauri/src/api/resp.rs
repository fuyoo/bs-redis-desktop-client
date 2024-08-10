use std::error::Error;
use serde::Serialize;

pub trait IntoResponse {
    fn into_response(self) -> tauri::Result<impl serde::Serialize>;
}
#[derive(Serialize, Debug)]
pub struct Response<T>
where
    T: serde::Serialize,
{
    pub code: i32,
    pub data: T,
    pub msg: Option<String>,
}
#[allow(refining_impl_trait)]
impl<T: serde::Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> tauri::Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}
#[macro_export] macro_rules! r_ok {
    ($data: expr,$msg:expr) => {
        Response {
            code: 0,
            data:$data,
            msg:$msg,
        }
    }
}

#[macro_export] macro_rules! r_404 {
    ($data: expr) => {
        Response {
            code: 4,
            data: $data,
            msg: Some("404 route not found.".to_string()),
        }
    }
}
#[macro_export] macro_rules! r_fail {
    ($msg:expr) => {
        Response {
            code: 3,
            data: (),
            msg:$msg,
        }
    }
}
#[macro_export] macro_rules! r_error {
    ($err:expr) => {
        Response {
            code: 5,
            data:(),
            msg:Some(format!("{:?}",$err)),
        }
    }
}
impl<T: serde::Serialize> Response<T> {
    pub fn ok(data: T, msg: Option<String>) -> Response<T> {
        r_ok!(data,msg)
    }
    pub fn fail(msg: Option<String>) -> Response<()> {
        r_fail!(msg)
    }

    pub fn error(err: impl Error) -> Response<()> {
        r_error!(err)
    }
}
