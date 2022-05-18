use std::process::Command;
#[cfg(target_os = "windows")]
fn main() {}

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