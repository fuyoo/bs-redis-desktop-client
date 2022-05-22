use anyhow::Error;
use sciter::{make_args, Value};

pub async fn create(data:String, cb:Value) -> Result<(),Error>{
    cb.call(None,&make_args!(data),None)?;
    Ok(())
}