#![feature(let_chains)]
#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

pub mod utils;
mod desktop;
pub mod response;
pub mod routes;
pub mod models;
pub mod structs;

extern crate log;
extern crate anyhow;

use std::error::Error;
use crate::utils::make_sure_single_case;

fn main() -> Result<(),Box<dyn Error>> {
    if cfg!(debug_assertions) {
        env_logger::init();
    }
    make_sure_single_case();
    fix_path_env::fix().expect("path error!");
    desktop::main()?;
    Ok(())
}
