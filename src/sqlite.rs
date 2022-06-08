use crate::app::app;
use anyhow::Result;
use std::path::PathBuf;
use rusqlite::Connection;

pub async fn init() -> Result<()> {
    let conn = create_connection().await?;
    conn.execute(
        "create table if not exists connections (
                id              TEXT PRIMARY KEY,
                name            TEXT NOT NULL,
                address         TEXT NOT NULL,
                port            TEXT DEFAULT '6379',
                username        TEXT DEFAULT '',
                password        TEXT DEFAULT ''
        )",
        [],
    )?;
    // 初始化系统信息表
    conn.execute(
        "create table if not exists sys_info(
                    id text primary key,
                    auto_refresh bool not null default false,
                    auto_refresh_time number not null default 3,
                    pubsub bool not null default true,
                    lang text not null default zh
                )",
        [],
    )?;
    Ok(())
}

pub async fn create_connection() -> Result<Connection> {
    let db = PathBuf::from(&app().lock().app_data_dir).join("data.db");
    let conn = rusqlite::Connection::open(db)?;
    Ok(conn)
}
