use std::{env, path::PathBuf};

pub fn get_os() -> String {
    match env::consts::OS {
        "linux" => "linux",
        "macos" => "mac-os",
        "windows" => "windows",
        _ => panic!("Unsupported OS!"),
    }
    .to_string()
}

pub fn get_arch() -> String {
    match env::consts::ARCH {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        "x86" => "x86",
        _ => panic!("Unsupported arch!"),
    }
    .to_string()
}

pub fn default_install_path(package_type: &str) -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get the home directory!")
        .join(".java")
        .join(package_type)
}
