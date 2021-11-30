/*! # sqlite连接管理模块*/
use crate::app::Response;
use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, result::Result::Ok};
use tauri::{command, Manager, Window};
use uuid::Uuid;
/// 创建数据库连接
fn create_sqlite_connection() -> Result<Connection> {
    let database = PathBuf::from(&crate::app::APP.lock().app_dir).join("app.db");
    let conn = Connection::open(database)?;
    Ok(conn)
}
/// 封装一个方法，获取连接
pub fn exec<F, T>(func: F) -> Result<T>
where
    F: FnOnce(&mut Connection) -> Result<T>,
{
    match create_sqlite_connection() {
        Ok(mut conn) => func(&mut conn),
        Err(e) => Err(e),
    }
}

/// 创建表
pub fn init_table() {
    let res = exec(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS connections (
                id              TEXT PRIMARY KEY,
                name            TEXT NOT NULL,
                address              TEXT NOT NULL,
                port            TEXT DEFAULT '6379',
                username        TEXT DEFAULT '',
                password             TEXT DEFAULT ''
        )",
            [],
        )?;
        // 初始化系统信息表
        conn.execute(
            "create table if not exists sys_info(
                    id text primary key,
                    auto_refresh bool not null default false,
                    auto_refresh_time number not null default 3,
                    pubsub bool not null default true
                )",
            [],
        )?;
        // 查询初始化数据是否存在
        let res: rusqlite::Result<String> =
            conn.query_row("select * from sys_info where id = '1'", [], |row| {
                row.get(0)
            });
        // 报错代表初始化数据不存在
        match res {
            Err(_) => {
                let res = conn.execute("insert into sys_info(id,auto_refresh,auto_refresh_time,pubsub) values('1',false,3,true)",[])?;
                if res == 0 {
                    panic!("init system info error")
                }
            }
            _ => {}
        }
        Ok(())
    });
    info!("init table {:?}", res);
}

/// 数据对应字段
#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: String,
    pub username: String,
    pub password: String,
}

/// 创建一个连接
#[command]
pub async fn connection_create(mut map: Map) -> Response<Option<bool>> {
    map.id = Uuid::new_v4().to_string();
    let res = exec(|conn| {
        let conn = conn.execute(
      "insert into connections(id,name,address,port,username,password) values(?1,?2,?3,?4,?5,?6)",
      params![
        map.id,
        map.name,
        map.address,
        map.port,
        map.username,
        map.password
      ],
    )?;
        Ok(conn > 0)
    });
    if let Ok(res) = res {
        if res {
            Response::new(200, Some(true), "操作成功！")
        } else {
            Response::new(300, None, "操作失败！")
        }
    } else {
        Response::new(300, None, "操作失败！")
    }
}

/// 删除一个连接
#[command]
pub async fn connection_delete(id: String) -> Response<bool> {
    let res = exec(|conn| {
        let mut stmt = conn.prepare("delete from connections where id =?1")?;
        let res = stmt.execute(params![id])?;
        Ok(res > 0)
    });
    info!("res -> {:?}", res);
    if let Ok(res) = res {
        if res {
            Response::new(200, true, "删除成功！")
        } else {
            Response::new(300, false, "删除失败！")
        }
    } else {
        Response::new(300, false, "删除失败！")
    }
}

/// 编辑一个连接
#[command]
pub async fn connection_edit(map: Map) -> Response<bool> {
    let _ = exec(|conn| {
        let _ = conn.execute(
            "replace into connections(id,name,address,port,password) values(?1,?2,?3,?4,?5)",
            params![map.id, map.name, map.address, map.port, map.password],
        )?;
        Ok(())
    });
    Response::new(200, true, "修改成功！")
}

/// 查询连接列表
#[command]
pub async fn connection_list() -> Response<Vec<Map>> {
    let res = exec(|conn| {
        let mut stmt = conn.prepare("select * from connections")?;
        let mut res = stmt.query(params![])?;
        let mut vec = vec![];
        while let Some(row) = res.next()? {
            vec.push(Map {
                id: row.get("id")?,
                name: row.get("name")?,
                address: row.get("address")?,
                port: row.get("port")?,
                username: row.get("username")?,
                password: row.get("password")?,
            })
        }
        Ok(vec)
    });
    if let Ok(records) = res {
        Response::new(200, records, "操作成功！")
    } else {
        Response::new(200, Vec::new(), "操作成功！")
    }
}

/// 更新系统信息
#[command]
pub async fn update_sys_info(
    window: Window,
    id: String,
    auto_refresh: bool,
    auto_refresh_time: isize,
    pubsub: bool,
) -> Response<bool> {
    let res = exec(|conn| {
        let res = conn.execute(
            "update sys_info set auto_refresh=?1,auto_refresh_time=?2,pubsub=?3 where id=?4",
            params![auto_refresh, auto_refresh_time, pubsub, id],
        )?;
        if res == 0 {
            return Err(anyhow!("操作失败！"));
        }
        Ok(())
    });

    match res {
        Ok(_) => {
            let _ = window.emit_all("sys_info_update", true);
            Response::new(200, true, "操作成功！")
        }
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysInfo {
    pub id: String,
    #[serde(rename = "autoRefresh")]
    pub auto_refresh: bool,
    #[serde(rename = "autoRefreshTime")]
    pub auto_refresh_time: isize,
    pub pubsub: bool,
}
/// 查询系统信息
#[command]
pub async fn query_sys_info() -> Response<Option<SysInfo>> {
    let res = exec(|conn| {
        let res = conn.query_row("select * from sys_info", [], |row| {
            Ok(SysInfo {
                id: row.get("id")?,
                auto_refresh: row.get("auto_refresh")?,
                auto_refresh_time: row.get("auto_refresh_time")?,
                pubsub: row.get("pubsub")?,
            })
        })?;
        Ok(res)
    });
    match res {
        Ok(res) => Response::new(200, Some(res), "查询成功！"),
        Err(e) => Response::new(200, None, &format!("{}", e)),
    }
}
