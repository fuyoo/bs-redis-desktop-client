use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionInfo{
    pub host: String,
    pub port: Option<String>,
    pub db: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionImpl {
    pub id: usize,
    pub name: String,
    pub node: Vec<ConnectionInfo>,
    pub cluster: Option<bool>
}