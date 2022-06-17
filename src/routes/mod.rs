use anyhow::Error;
use sciter::make_args;
use crate::Action;
use crate::response::{Body, Response};

mod connection;
mod rdb_connection;


pub async fn dispatch(action: Action) -> Result<(), Error> {
    let res = match action.path.as_str() {
        "/connection/create" => connection::create(&action.data).await?.into_response()?,
        "/connection/list" => connection::list().await?.into_response()?,
        "/connection/delete" => connection::delete(&action.data).await?.into_response()?,
        "/connection/update" => connection::update(&action.data).await?.into_response()?,
        "/connection/test" => connection::test_connection(&action.data).await?.into_response()?,
        "/rdb/connect" => rdb_connection::change_active_client(&action.data).await?.into_response()?,
        "/rdb/change_db" => rdb_connection::change_active_client_db(&action.data).await?.into_response()?,
        "/rdb/info" => rdb_connection::info(&action.data).await?.into_response()?,
        "/rdb/cfg" => rdb_connection::cfg(&action.data).await?.into_response()?,
        _ => Response::<Option<&str>>::new(404, None, "not found!").into_response()?
    };
    action.cb.call(None, &make_args!(res), None)?;
    Ok(())
}

