extern crate windows_exe_info;

#[cfg(target_os = "windows")]
fn main() {
    windows_exe_info::icon::icon_ico(std::path::Path::new("./images/icon.ico"));
}

#[cfg(not(target_os = "windows"))]
fn main() {}
