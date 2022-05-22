use std::process::Command;
#[cfg(not(target_os = "windows"))]
fn main() {
    Command::new("packfolder")
        .arg("views")
        .arg("src/resource.rc")
        .arg("-binary")
        .output()
        .unwrap();
    println!("cargo:rerun-if-changed=/views");
}

#[cfg(target_os = "windows")]
fn main() {
    embed_resource::compile("windows_platform_resource/build.rc");
    Command::new("packfolder")
        .arg("views")
        .arg("src/resource.rc")
        .arg("-binary")
        .output()
        .unwrap();
    println!("cargo:rerun-if-changed=/views");
}