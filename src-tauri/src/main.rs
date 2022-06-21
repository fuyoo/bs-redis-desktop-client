#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod app;
mod rdb;
mod sqlite;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rusqlite;
#[macro_use]
extern crate anyhow;


fn main() {
    if cfg!(debug_assertions) {
        env_logger::init();
    }
    fix_path_env::fix().expect("path error!");
    app::lock_single();
    let _app = tauri::Builder::default()
        .setup(|_app| {
            #[cfg(target_os = "windows")]
            app::webview2_is_installed();
            if !app::init_app_dir() {
                panic!("工作目录初始化失败！");
            }
            sqlite::init_table();
            Ok(())
        })
        .menu(app::create_app_menu())
        .on_menu_event(app::handle_event_app_menu_event)
        .system_tray(app::create_try())
        .on_system_tray_event(app::handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            app::listen_single,
            sqlite::connection_list,
            sqlite::connection_delete,
            sqlite::connection_create,
            sqlite::connection_edit,
            sqlite::update_sys_info,
            sqlite::query_sys_info,
            rdb::test_redis_connection,
            rdb::set_redis_config,
            rdb::get_redis_db,
            rdb::get_redis_info,
            rdb::get_redis_keys,
            rdb::get_redis_key_info,
            rdb::set_new_key,
            rdb::rename_key,
            rdb::set_key_ttl,
            rdb::del_key,
            rdb::get_string_type_data_as_string,
            rdb::get_string_type_data_as_blob,
            rdb::get_list_data,
            rdb::modify_list_data_by_index,
            rdb::delete_list_data_by_value,
            rdb::push_data_to_exists_key_list,
            rdb::slice_list,
            rdb::scan_hash_data,
            rdb::del_hash_field,
            rdb::modify_hash_value_by_field,
            rdb::add_new_field_to_exists_hash,
            rdb::scan_set_data,
            rdb::del_set_member,
            rdb::add_new_member_to_exists_set,
            rdb::scan_zset_data,
            rdb::add_new_member_to_exists_zset,
            rdb::del_member_from_zset,
            rdb::modify_zset_value_by_member,
            rdb::create_channel,
            rdb::remove_channel,
            rdb::sub_stop_all,
            rdb::get_channel_list,
            rdb::publish_msg
        ])
        .build(tauri::generate_context!())
        .expect("发生了未知错误！");

    _app.run(app::handle_app_event);
}
