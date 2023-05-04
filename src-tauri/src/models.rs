use crate::utils::get_connection;
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row};

#[derive(Debug, Serialize, Deserialize)]
pub struct Connections {
    pub id: Option<String>,
    pub name: String,
    pub address: String,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub cluster: Option<bool>,
    pub nodes: Option<String>,
    pub proxy: Option<bool>,
    #[serde(rename = "proxyKeyType")]
    pub proxy_key_type: Option<i32>,
    #[serde(rename = "proxyUsername")]
    pub proxy_username: Option<String>,
    #[serde(rename = "proxyAddress")]
    pub proxy_address: Option<String>,
    #[serde(rename = "proxyPort")]
    pub proxy_port: Option<i32>,
    #[serde(rename = "proxyPassword")]
    pub proxy_password: Option<String>,
    #[serde(rename = "proxyFilePath")]
    pub proxy_file_path: Option<String>,

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
            cluster: row.get("cluster"),
            nodes: row.get("nodes"),
            proxy: row.get("proxy"),
            proxy_key_type: row.get("proxy_key_type"),
            proxy_username: row.get("proxy_username"),
            proxy_address: row.get("proxy_address"),
            proxy_port: row.get("proxy_port"),
            proxy_password: row.get("proxy_password"),
            proxy_file_path: row.get("proxy_file_path"),
        }
    }
    pub async fn delete(id: &str) -> Result<bool> {
        let mut conn = get_connection().await?;
        let res = sqlx::query("delete from connections where id =?1")
            .bind(id)
            .execute(&mut conn)
            .await?;
        Ok(res.rows_affected() > 0)
    }
    pub async fn get(id: &str) -> Result<Self> {
        let mut client = get_connection().await?;
        let row = sqlx::query("select * from connections where id = ?1")
            .bind(id)
            .fetch_one(&mut client)
            .await?;
        Ok(Connections::bind(&row))
    }
    pub async fn insert_or_update(self) -> Result<bool> {
        debug!("{:#?}",self);
        let mut client = get_connection().await?;
        let res = sqlx::query(
            r"
        replace into connections(
        id,
        name,
        address,
        port,
        username,
        password,
        cluster,
        nodes,
        proxy,
        proxy_key_type,
        proxy_username,
        proxy_address,
        proxy_port,
        proxy_password,
        proxy_file_path
        )
        values (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)
    ",
        )
            .bind(self.id)
            .bind(self.name)
            .bind(self.address)
            .bind(self.port)
            .bind(self.username)
            .bind(self.password)
            .bind(self.cluster)
            .bind(self.nodes)
            .bind(self.proxy)
            .bind(self.proxy_key_type)
            .bind(self.proxy_username)
            .bind(self.proxy_address,)
            .bind(self.proxy_port)
            .bind(self.proxy_password)
            .bind(self.proxy_file_path)
            .execute(&mut client)
            .await?;
        Ok(res.rows_affected() > 0)
    }
    pub async fn get_all() -> Result<Vec<Self>> {
        let mut client = get_connection().await?;
        let rows = sqlx::query("select * from connections")
            .fetch_all(&mut client)
            .await?
            .iter()
            .map(|row| Connections::bind(row))
            .collect::<Vec<Connections>>();
        Ok(rows)
    }
}
