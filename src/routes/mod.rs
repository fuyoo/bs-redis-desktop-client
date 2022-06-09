use anyhow::Error;
use sciter::make_args;
use crate::Action;
use crate::response::{Body, Response};

mod connection;


pub async fn dispatch(action: Action) -> Result<(), Error> {
    let res = match action.path.as_str() {
        "/connection/create" => connection::create(&action.data).await?.into_response()?,
        "/connection/list" => connection::search(&action.data).await?.into_response()?,
        _ => Response::<Option<&str>>::new(404, None, "not found!").into_response()?
    };
    action.cb.call(None, &make_args!(res), None)?;
    Ok(())
}

