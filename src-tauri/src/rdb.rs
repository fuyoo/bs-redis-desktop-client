/*! # redis 连接管理模块*/
use crate::app::Response;
use parking_lot::Mutex;
use redis::{cmd, Commands, Connection, Value};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, option::Option::Some, sync::Arc};
use tauri::{command, Window};
lazy_static! {
    ///实例化一个常驻的配置信息
    pub static ref CFG: Mutex<RedisConfig> = Mutex::new(RedisConfig::new());
    pub static ref MS: Mutex<Handle> = Mutex::new(Handle::new());
}
/// redis配置信息
#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    ///连接地址
    address: String,
    ///连接端口
    port: String,
    ///连接密码
    password: String,
    ///连接用户名
    username: String,
    ///连接数据库
    db: String,
}

impl RedisConfig {
    /// 新建一个结构体
    pub fn new() -> RedisConfig {
        RedisConfig {
            address: "".to_string(),
            port: "".to_string(),
            password: "".to_string(),
            username: "".to_string(),
            db: "".to_string(),
        }
    }
    /// 解析结构体为redis连接信息
    pub fn parse_connection_info() -> redis::ConnectionInfo {
        // 初始化用户
        let username = if &CFG.lock().username != "" {
            Some(String::from(&CFG.lock().username))
        } else {
            None
        };
        // 初始化密码
        let password = if &CFG.lock().password != "" {
            Some(String::from(&CFG.lock().password))
        } else {
            None
        };
        // 初始化端口
        let port = if &CFG.lock().port != "" {
            String::from(&CFG.lock().port)
                .parse::<u16>()
                .unwrap_or(6379)
        } else {
            6379
        };
        // 初始化数据库
        let db = if &CFG.lock().db != "" {
            String::from(&CFG.lock().db).parse::<i64>().unwrap_or(0)
        } else {
            0
        };
        // 生成连接信息
        redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(String::from(&CFG.lock().address), port),
            redis: redis::RedisConnectionInfo {
                db,
                username,
                password,
            },
        }
    }
    /// 设置地址
    pub fn set_address(&mut self, para: &str) {
        self.address = para.into();
    }
    /// 设置端口
    pub fn set_port(&mut self, para: String) {
        self.port = para;
    }
    /// 设置数据库
    pub fn set_db(&mut self, para: String) {
        self.db = para;
    }
    /// 设置用户名
    pub fn set_username(&mut self, para: &str) {
        self.username = para.into();
    }
    /// 设置密码
    pub fn set_password(&mut self, para: &str) {
        self.password = para.into();
    }
}

/// 获取redis连接
pub fn get_connection<F, T>(func: F) -> anyhow::Result<T>
where
    F: FnOnce(&mut Connection) -> anyhow::Result<T>,
{
    let client = redis::Client::open(RedisConfig::parse_connection_info())?;
    let mut connection = client.get_connection()?;
    connection.set_write_timeout(Some(std::time::Duration::from_secs(1)))?;
    let res = func(&mut connection)?;
    Ok(res)
}

/// 测试连接配置信息
#[command]
pub async fn test_redis_connection(
    address: String,
    port: String,
    username: String,
    password: String,
) -> Response<bool> {
    // 初始化用户名
    let username = if username == "" { None } else { Some(username) };
    // 初始化密码
    let password = if password == "" { None } else { Some(password) };
    // 初始化端口
    let port = if port == "" {
        0
    } else {
        port.parse().unwrap_or(0)
    };
    // 生成连接信息
    let info = redis::ConnectionInfo {
        addr: redis::ConnectionAddr::Tcp(address, port),
        redis: redis::RedisConnectionInfo {
            db: 0,
            password,
            username,
        },
    };
    // 创建客户端
    let client = redis::Client::open(info);
    // 校验
    if let Ok(client) = client {
        if let Ok(_) = client.get_connection() {
            Response::new(200, true, "连接成功！")
        } else {
            Response::new(300, false, "连接失败！")
        }
    } else {
        Response::new(300, false, "连接失败！")
    }
}

/// 设置redis的连接信息
#[command]
pub async fn set_redis_config(
    address: String,
    db: String,
    port: String,
    password: String,
    username: String,
) -> Response<bool> {
    if address != "" {
        let _ = &CFG.lock().set_address(&address);
    }
    if db != "" {
        let _ = &CFG.lock().set_db(db);
    }
    if port != "" {
        let _ = &CFG.lock().set_port(port);
    }
    let _ = &CFG.lock().set_password(&password);
    let _ = &CFG.lock().set_username(&username);
    if let Ok(_) = get_connection(|_| Ok(())) {
        Response::new(200, true, "连接成功！")
    } else {
        Response::new(300, false, "连接失败！")
    }
}

/// 获取redis数据库个数
#[command]
pub async fn get_redis_db() -> Response<String> {
    let res = get_connection(|conn| {
        let res = cmd("config")
            .arg("get")
            .arg("databases")
            .query::<Value>(conn)?;
        match res {
            Value::Bulk(res) => {
                let mut vals = vec![];
                for val in res {
                    match val {
                        Value::Data(data) => {
                            if let Ok(res) = String::from_utf8(data) {
                                vals.push(res);
                            }
                        }
                        _ => {}
                    }
                }
                return Ok(String::from(&vals[vals.len() - 1]));
            }
            _ => {}
        }
        Ok("0".to_string())
    });
    if let Ok(res) = res {
        Response::new(200, res, "查询成功！")
    } else {
        Response::new(300, "0".to_string(), "连接失败！")
    }
}

/// 获取redis的信息
#[command]
pub async fn get_redis_info() -> Response<String> {
    let res = get_connection(|conn| {
        let res = cmd("info").query::<String>(conn)?;
        Ok(res)
    });
    if let Ok(res) = res {
        Response::new(200, res, "查询成功！")
    } else {
        Response::new(300, String::new(), "查询失败！")
    }
}

/// 获取redis的所有key
#[command]
pub async fn get_redis_keys(query: String, size: String) -> Response<Vec<String>> {
    if !query.contains("*") {
        let mut arr = vec![];

        let res = get_connection(|conn| {
            let res = cmd("keys").arg(query).query::<Value>(conn)?;
            match res {
                Value::Bulk(res) => {
                    for item in res {
                        match item {
                            Value::Data(data) => {
                                arr.push(
                                    String::from_utf8(data).unwrap_or(format!("")),
                                );
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            Ok(arr)
        });

        if let Ok(res) = res {
            return Response::new(200, res, "查询成功！");
        } else {
            return Response::new(200, vec![], "查询成功！");
        }
    }

    let res = get_connection(|conn| {
        let mut cursor = format!("-1");
        let mut arr = vec![];
        'outer: while cursor != "0" {
            if cursor == "-1" {
                cursor = format!("0");
            }
            let res = cmd("scan")
                .arg(&cursor)
                .arg("match")
                .arg(&query)
                .arg("count")
                .arg("500")
                .query::<Value>(conn)?;
            match res {
                Value::Bulk(res) => {
                    for item in res {
                        match item {
                            Value::Data(data) => {
                                cursor = String::from_utf8(data).unwrap_or(format!("0"))
                            }
                            Value::Bulk(res) => {
                                for item in res {
                                    match item {
                                        Value::Data(data) => {
                                            arr.push(
                                                String::from_utf8(data).unwrap_or(format!("")),
                                            );

                                            let size = size.parse::<usize>().unwrap_or(0);
                                            if size > 0 && arr.len() >= size {
                                                break 'outer;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(arr)
    });
    if let Ok(res) = res {
        Response::new(200, res, "查询成功！")
    } else {
        Response::new(200, vec![], "查询成功！")
    }
}

/// 键的信息
#[derive(Deserialize, Serialize, Debug)]
pub struct KeyInfo {
    pub t: String,
    pub size: i64,
    pub ttl: i64,
    pub status: bool,
}

/// 获取键的相关信息
#[command]
pub async fn get_redis_key_info(key: String) -> Response<Option<KeyInfo>> {
    let res = get_connection(|conn| {
        let mut redis_version = 0;
        let res = cmd("info").arg("server").query::<Value>(conn)?;

        match res {
            Value::Data(data) => {
                if let Ok(str) = String::from_utf8(data) {
                    let str = str.split("\r\n");
                    let mut version = "";
                    for s in str {
                        if s.contains("redis_version") {
                            version = s;
                            break;
                        }
                    }
                    let ver = version.split(":").collect::<Vec<&str>>();
                    let ver = ver[1].split(".").collect::<Vec<&str>>();
                    match ver[0].parse::<i32>() {
                        Ok(res) => {
                            redis_version = res;
                        }
                        Err(err) => {
                            return Err(anyhow!("{}", err));
                        }
                    }
                }
            }
            _ => {}
        }
        let mut info = KeyInfo {
            t: "".to_string(),
            size: 0,
            ttl: 0,
            status: true,
        };
        if redis_version > 3 {
            let res = redis::pipe()
                .cmd("type")
                .arg(&key)
                .cmd("memory")
                .arg("usage")
                .arg(&key)
                .cmd("pttl")
                .arg(&key)
                .query::<Value>(conn)?;
            match res {
                Value::Bulk(data) => {
                    match &data[0] {
                        Value::Status(data) => info.t = data.to_string(),
                        _ => info.status = false,
                    }
                    match &data[1] {
                        Value::Int(data) => info.size = data.clone(),
                        _ => info.status = false,
                    }

                    match &data[2] {
                        Value::Int(data) => info.ttl = data.clone(),
                        _ => info.status = false,
                    }
                }
                _ => {}
            }
        } else {
            let res = redis::pipe()
                .cmd("type")
                .arg(&key)
                .cmd("pttl")
                .arg(&key)
                .query::<Value>(conn)?;
            match res {
                Value::Bulk(data) => {
                    match &data[0] {
                        Value::Status(data) => info.t = data.to_string(),
                        _ => info.status = false,
                    }
                    match &data[1] {
                        Value::Int(data) => info.ttl = data.clone(),
                        _ => info.status = false,
                    }
                }
                _ => {}
            }
        }
        Ok(info)
    });
    match res {
        Ok(res) => Response::new(200, Some(res), "查询成功！"),
        Err(err) => {
            info!("err->{:?}", err);
            Response::new(300, None, "查询失败！")
        }
    }
}

/// 新增键
#[command]
pub async fn set_new_key(
    key: String,
    t: String,
    value: String,
    time_type: String,
    expire: String,
    field: String,
    member: String,
) -> Response<bool> {
    let res = get_connection(move |conn| {
        let res: bool = conn.exists(&key)?;
        if res {
            return Err(anyhow!("key 已存在！"));
            //return anyhow::Result::from_error(std::error::Error::fmt("err key exists"));
        }

        match t.as_str() {
            "string" => {
                if expire == "" {
                    conn.set(&key, &value)?;
                } else {
                    let expire = expire.parse::<usize>().unwrap_or(0);
                    if time_type == "pttl" {
                        conn.pset_ex(&key, &value, expire)?;
                    } else {
                        conn.set_ex(&key, &value, expire)?
                    }
                }
            }
            "hash" => {
                conn.hset(&key, &field, &value)?;
            }
            "zset" => {
                conn.zadd(&key, &member, &value)?;
            }
            "set" => {
                conn.sadd(&key, &value)?;
            }
            "list" => {
                conn.lpush(&key, &value)?;
            }
            _ => {
                return Err(anyhow!("不支持的数据格式"));
            }
        }
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "操作成功！"),
        Err(e) => Response::new(300, false, format!("{}", e).as_str()),
    }
}

/// 重命名
#[command]
pub async fn rename_key(key: String, new_key: String) -> Response<bool> {
    let res = get_connection(|conn| {
        let res: bool = conn.exists(&new_key)?;
        if res {
            return Err(anyhow!("key 已存在！"));
            //return anyhow::Result::from_error(std::error::Error::fmt("err key exists"));
        }
        conn.rename(&key, &new_key)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, false, "重命名成功！"),
        Err(err) => Response::new(300, false, format!("{}", err).as_str()),
    }
}

/// 设置ttl
#[command]
pub async fn set_key_ttl(key: String, t: String, expire: String) -> Response<bool> {
    let res = get_connection(|conn| {
        if expire == "-1" {
            conn.persist(&key)?;
            return Ok(());
        }
        let expire = expire.parse::<usize>()?;
        match t.as_str() {
            "ttl" => {
                conn.expire(&key, expire)?;
            }
            "pttl" => {
                conn.pexpire(&key, expire)?;
            }
            _ => {
                return Err(anyhow!("时间类型错误！"));
            }
        }
        Ok(())
    });

    match res {
        Ok(_) => Response::new(200, true, "更新过期时间成功！"),
        Err(e) => Response::new(300, false, format!("{}", e).as_str()),
    }
}

/// 删除key
#[command]
pub async fn del_key(key: String) -> Response<bool> {
    let res = get_connection(|conn| {
        conn.del(&key)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "删除成功！"),
        Err(e) => Response::new(300, false, format!("{}", e).as_str()),
    }
}

/// 获取字符串类型的数据
#[command]
pub async fn get_string_type_data_as_string(key: String) -> Response<String> {
    let res = get_connection(|conn| {
        let res: String = conn.get(&key)?;
        Ok(res)
    });
    match res {
        Ok(res) => Response::new(200, res, "查询成功！"),
        Err(e) => Response::new(300, "".to_string(), format!("{}", e).as_str()),
    }
}

/// 以blob的方式获取数据
#[command]
pub async fn get_string_type_data_as_blob(key: String) -> Response<Option<Vec<u8>>> {
    let res = get_connection(|conn| {
        let res: Value = conn.get(&key)?;
        match res {
            Value::Data(data) => {
                return Ok(data);
            }
            _ => {}
        }
        Ok(vec![])
    });

    match res {
        Ok(res) => Response::new(200, Some(res), "success"),
        Err(err) => Response::new(300, None, format!("{}", err).as_str()),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListData {
    pub page: isize,
    pub total: isize,
    pub size: isize,
    pub records: Vec<String>,
}

/// list数据
#[command]
pub async fn get_list_data(key: String, page: isize, size: isize) -> Response<Option<ListData>> {
    let res = get_connection(|conn| {
        let mut values = vec![];
        let res: Value = conn.lrange(&key, size * (page - 1), size * (page - 1) + size - 1)?;
        match res {
            Value::Bulk(data) => {
                for item in data {
                    match item {
                        Value::Data(data) => {
                            values.push(String::from_utf8(data)?);
                        }
                        _ => {
                            values.push("不支持的数据类型格式！".to_string());
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(ListData {
            page,
            total: conn.llen(&key)?,
            size,
            records: values,
        })
    });

    match res {
        Ok(data) => Response::new(200, Some(data), "查询成功！"),
        Err(e) => Response::new(300, None, format!("{}", e).as_str()),
    }
}

///根据索引修改list数据
#[command]
pub async fn modify_list_data_by_index(key: String, index: isize, value: String) -> Response<bool> {
    let res = get_connection(|conn| {
        conn.lset(&key, index, value)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "操作成功！"),
        Err(e) => Response::new(300, false, format!("{}", e).as_str()),
    }
}

/// 删除list中与value参数一致的值
#[command]
pub async fn delete_list_data_by_value(key: String, value: String) -> Response<bool> {
    let res = get_connection(|conn| {
        let len: isize = conn.llen(&key)?;
        if len == 1 {
            return Err(anyhow!("{}", "最后一条数据不能删除，请直接删除key!"));
        }
        conn.lrem(&key, 0, &value)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "删除成功！"),
        Err(e) => Response::new(300, false, format!("{}", e).as_str()),
    }
}

/// 向一个已经存在的list中插入值
#[command]
pub async fn push_data_to_exists_key_list(
    key: String,
    value: String,
    how: String,
) -> Response<bool> {
    let res = get_connection(|conn| match how.as_str() {
        "r" => {
            conn.rpush(&key, &value)?;
            Ok(())
        }
        "l" => {
            conn.lpush(&key, &value)?;
            Ok(())
        }
        _ => Err(anyhow!("{}", "插入方向不支持！")),
    });
    match res {
        Ok(_) => Response::new(200, true, "插入值成功！"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

/// 裁剪列表
#[command]
pub async fn slice_list(key: String, start: isize, end: isize) -> Response<bool> {
    let res = get_connection(|conn| {
        let len: isize = conn.llen(&key)?;
        if start < 0 {
            return Err(anyhow!("开始值不能小于0"));
        }
        if end > len {
            return Err(anyhow!("最大裁剪长度为{}", len));
        }
        if start >= end {
            return Err(anyhow!("裁剪长度错误"));
        }
        conn.ltrim(&key, start, end)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "裁剪成功！"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HashData {
    pub cursor: isize,
    pub records: Vec<HashMap<String, Vec<u8>>>,
}

#[command]
pub async fn scan_hash_data(
    key: String,
    cursor: isize,
    pattern: String,
    count: isize,
) -> Response<Option<HashData>> {
    let res = get_connection(|conn| {
        let res = cmd("hscan")
            .arg(&key)
            .arg(cursor)
            .arg("match")
            .arg(&pattern)
            .arg("count")
            .arg(count)
            .query::<Value>(conn)?;
        let mut hash_data = HashData {
            cursor: 0,
            records: vec![],
        };
        match res {
            Value::Bulk(bulk) => {
                for item in bulk {
                    match item {
                        Value::Data(cursor) => {
                            let cursor = String::from_utf8(cursor)?.parse::<isize>()?;
                            hash_data.cursor = cursor
                        }
                        Value::Bulk(bulk) => {
                            let mut key = vec![];
                            let mut val = vec![];
                            for (k, v) in bulk.iter().enumerate() {
                                match v {
                                    Value::Data(data) => {
                                        if k % 2 == 0 {
                                            key.push(data.clone());
                                        } else {
                                            val.push(data.clone());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            for (k, v) in key.iter().enumerate() {
                                let mut map = HashMap::new();
                                map.insert("key".to_string(), v.clone());
                                map.insert("value".to_string(), val[k].clone());
                                hash_data.records.push(map);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(hash_data)
    });
    match res {
        Ok(data) => Response::ok(Some(data), "查询成功！"),
        Err(e) => Response::err(None, &format!("{}", e)),
    }
}

/// 删除字段
#[command]
pub async fn del_hash_field(key: String, field: String) -> Response<bool> {
    let res = get_connection(|conn| {
        conn.hdel(&key, &field)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "删除字段操作成功"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

#[command]
pub async fn modify_hash_value_by_field(
    key: String,
    field: String,
    value: String,
) -> Response<bool> {
    let res = get_connection(|conn| {
        conn.hset(key, field, value)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "修改成功"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

/// 在已存在的hash中插入一个字段
#[command]
pub async fn add_new_field_to_exists_hash(
    key: String,
    field: String,
    value: String,
) -> Response<bool> {
    let res = get_connection(|conn| {
        // check field is exists?
        let res: bool = conn.hexists(&key, &field)?;
        if res {
            return Err(anyhow!("字段已存在！"));
        }
        conn.hset(key, field, value)?;
        Ok(())
    });

    match res {
        Ok(_) => Response::new(200, true, "添加字段成功!"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetData {
    pub cursor: isize,
    pub records: Vec<HashMap<String, String>>,
}

///获取set数据
#[command]
pub async fn scan_set_data(
    key: String,
    cursor: isize,
    pattern: String,
    count: isize,
) -> Response<Option<SetData>> {
    let res = get_connection(|conn| {
        let res = cmd("sscan")
            .arg(key)
            .arg(cursor)
            .arg("match")
            .arg(pattern)
            .arg("count")
            .arg(count)
            .query::<Value>(conn)?;
        let mut set_data = SetData {
            cursor,
            records: vec![],
        };
        match res {
            Value::Bulk(bulk) => {
                for item in bulk {
                    match item {
                        Value::Data(data) => {
                            set_data.cursor = String::from_utf8(data)?.parse::<isize>()?;
                        }
                        Value::Bulk(data) => {
                            for v in data {
                                match v {
                                    Value::Data(data) => {
                                        let mut map = HashMap::new();
                                        map.insert("member".to_string(), String::from_utf8(data)?);
                                        set_data.records.push(map);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(set_data)
    });
    match res {
        Ok(data) => Response::new(200, Some(data), "修改成功"),
        Err(e) => Response::new(300, None, &format!("{}", e)),
    }
}

/// 删除set中的一个数据
#[command]
pub async fn del_set_member(key: String, member: String) -> Response<bool> {
    let res = get_connection(|conn| {
        conn.srem(&key, member)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "删除成功！"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

/// 在一个已存在的set中添加成员
#[command]
pub async fn add_new_member_to_exists_set(key: String, member: String) -> Response<bool> {
    let res = get_connection(|conn| {
        // member is exists?
        if conn.sismember(&key, &member)? {
            return Err(anyhow!("成员已存在！"));
        }
        conn.sadd(&key, member)?;
        Ok(())
    });

    match res {
        Ok(_) => Response::new(200, true, "添加成功！"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZsetData {
    pub cursor: isize,
    pub records: Vec<HashMap<String, String>>,
}

/// 查询zset数据
#[command]
pub async fn scan_zset_data(
    key: String,
    cursor: isize,
    pattern: String,
    count: isize,
) -> Response<Option<ZsetData>> {
    let res = get_connection(|conn| {
        let res = cmd("zscan")
            .arg(key)
            .arg(cursor)
            .arg("match")
            .arg(pattern)
            .arg("count")
            .arg(count)
            .query::<Value>(conn)?;
        let mut zset_data = ZsetData {
            cursor: 0,
            records: vec![],
        };
        match res {
            Value::Bulk(bulk) => {
                for v in bulk {
                    match v {
                        Value::Data(data) => {
                            zset_data.cursor = String::from_utf8(data.clone())?.parse()?;
                        }
                        Value::Bulk(bulk) => {
                            let mut members = vec![];
                            let mut scores = vec![];
                            for (k, v) in bulk.iter().enumerate() {
                                match v {
                                    Value::Data(data) => {
                                        if k % 2 == 0 {
                                            members.push(data);
                                        } else {
                                            scores.push(data);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            for (k, v) in members.iter().enumerate() {
                                let mut map = HashMap::new();
                                map.insert("member".to_string(), String::from_utf8((*v).clone())?);
                                map.insert(
                                    "score".to_string(),
                                    String::from_utf8(scores[k].clone())?,
                                );
                                zset_data.records.push(map);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };
        Ok(zset_data)
    });
    match res {
        Ok(data) => Response::new(200, Some(data), "查询成功！"),
        Err(e) => Response::new(300, None, &format!("{}", e)),
    }
}

/// 新增
#[command]
pub async fn add_new_member_to_exists_zset(
    key: String,
    member: String,
    score: isize,
) -> Response<bool> {
    let res = get_connection(|conn| {
        // member is exists?
        let res: Value = conn.zrank(&key, &member)?;
        match res {
            Value::Int(_) => {
                return Err(anyhow!("成员已存在！"));
            }
            _ => {}
        }
        conn.zadd(key, member, score)?;

        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "添加成功！"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

/// 删除zset的一个成员
#[command]
pub async fn del_member_from_zset(key: String, member: String) -> Response<bool> {
    let res = get_connection(|conn| {
        conn.zrem(key, member)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "删除成功！"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

/// 修改zset内member的数据
#[command]
pub async fn modify_zset_value_by_member(
    key: String,
    member: String,
    score: String,
) -> Response<bool> {
    let res = get_connection(|conn| {
        conn.zadd(key, member, score)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "修改成功"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}

/// 通道监听数据结构体
pub struct Handle {
    pub handles: HashMap<String, bool>,
}

impl Handle {
    fn new() -> Handle {
        Handle {
            handles: HashMap::new(),
        }
    }

    fn add(&mut self, k: String, stop: bool) -> bool {
        self.handles.insert(k, stop);
        true
    }

    fn stop(&mut self, k: String) -> bool {
        if let Some(ok) = self.handles.insert(k, true) {
            !ok
        } else {
            false
        }
    }

    fn remove(&mut self, k: String) -> bool {
        if let Some(ok) = self.handles.remove(&k) {
            ok
        } else {
            false
        }
    }

    fn stop_all(&mut self) {
        let mut ks = vec![];
        for (k, _) in &self.handles {
            ks.push(format!("{}", k));
        }
        for k in ks {
            self.handles.insert(k, true);
        }
    }
    fn status(&mut self, k: String) -> bool {
        if let Some(ok) = self.handles.get(&k) {
            *ok == true
        } else {
            false
        }
    }

    fn channel_exists(&mut self, k: String) -> bool {
        // check is stop? if stop remove it
        if self.status(k.clone()) == true {
            self.remove(k.clone());
        }
        let mut b = false;
        for (key, _) in &self.handles {
            if key.to_string() == k {
                b = true;
                break;
            }
        }
        b
    }
}

/// 创建通道并监听数据
#[command]
pub async fn create_channel(window: Window, name: String) -> Response<bool> {
    let channel = format!("{}", &name);
    let address = Arc::new(format!("{}", &CFG.lock().address));
    let port = Arc::new(format!("{}", &CFG.lock().port));
    let always = address.clone();
    let always_port = port.clone();
    if MS
        .lock()
        .channel_exists(format!("{}-{}", &always, &channel))
    {
        return Response::new(300, false, "监听通道已存在！");
    }
    std::thread::spawn(move || {
        tauri::async_runtime::block_on(async move {
            let _: Result<(), anyhow::Error> = get_connection(|conn| {
                let mut ps = conn.as_pubsub();
                ps.subscribe(&name)?;
                let always = always.clone();
                loop {
                    let msg = ps.get_message()?;
                    let data: String = msg.get_payload()?;
                    let channel = msg.get_channel_name().to_string();
                    let k = format!("{}-{}", &always, &channel);
                    let status = MS.lock().status(k.clone());
                    if status == true {
                        MS.lock().remove(k);
                        break;
                    };
                    let mut map = HashMap::new();
                    map.insert("address", format!("{}", always));
                    map.insert("channel", channel);
                    map.insert("data", data);
                    map.insert("port", format!("{}", always_port));
                    window.emit("pubsub", map).unwrap();
                }
                Ok(())
            });
        });
    });

    let ok = MS.lock().add(format!("{}-{}", address, channel), false);
    if true == ok {
        Response::new(200, true, "通道创建成功！")
    } else {
        Response::new(300, false, "通道创建失败！")
    }
}

/// 移除通道
#[command]
pub async fn remove_channel(channel: String) -> Response<bool> {
    let ok = MS.lock().stop(channel);
    if true == ok {
        Response::new(200, true, "取消监听成功！")
    } else {
        Response::new(300, false, "取消监听失败！")
    }
}

/// 获取通道列表
#[command]
pub async fn get_channel_list() -> Response<Vec<HashMap<String, String>>> {
    let mut data = vec![];
    for (k, v) in MS.lock().handles.iter() {
        if v == &true {
            continue;
        }
        let mut map = HashMap::new();
        map.insert("key".to_string(), format!("{}", k));
        map.insert("stop".to_string(), format!("{}", v));
        data.push(map);
    }
    Response::new(200, data, "查询成功！")
}
/// 停止所有的订阅事件
#[command]
pub async fn sub_stop_all() -> Response<bool> {
    MS.lock().stop_all();
    Response::new(200, true, "停止成功！")
}

#[command]
pub async fn publish_msg(channel: String, data: String) -> Response<bool> {
    let res = get_connection(|conn| {
        info!("index -> looping");
        let _ = cmd("publish").arg(channel).arg(data).query::<Value>(conn)?;
        Ok(())
    });
    match res {
        Ok(_) => Response::new(200, true, "数据发布成功！"),
        Err(e) => Response::new(300, false, &format!("{}", e)),
    }
}
