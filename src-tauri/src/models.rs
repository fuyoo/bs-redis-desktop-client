use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Connections {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

impl Connections {
    pub fn bind(row: &SqliteRow) -> Self {
        Connections {
            id: row.get("id"),
            name: row.get("name"),
            address: row.get("address"),
            port: row.get("port"),
            username: row.get("username"),
            password: row.get("password"),
        }
    }
}