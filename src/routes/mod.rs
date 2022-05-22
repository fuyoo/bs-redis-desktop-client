use anyhow::Error;
use crate::Action;

mod database;

pub async fn dispatch(action:Action)-> Result<(),Error> {
    match action.path.as_str() {
        "/create" => {
            database::create(action.data,action.cb).await?;
        }
        _ => {
            println!("none");
        }
    }
    Ok(())
}

