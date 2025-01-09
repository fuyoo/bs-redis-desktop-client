// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // below code aim to embed Info.plist file,grant some permission to app
    // such as audio,microphone,and so on.
    #[cfg(debug_assertions)]
    embed_plist::embed_info_plist!("../Info.plist");
    let _ = bsrdc::run();
}
