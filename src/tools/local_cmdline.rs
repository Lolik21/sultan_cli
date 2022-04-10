use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::Stdio;

use crate::config::EmulatorCfg;

pub struct SdkManager {
    path: PathBuf,
    sdk_root: Option<PathBuf>,
}

pub struct AvdManager {
    path: PathBuf,
}

impl SdkManager {
    pub fn new(path: &PathBuf, sdk_root: Option<PathBuf>) -> SdkManager {
        return SdkManager {
            path: path.clone(),
            sdk_root,
        };
    }

    pub fn download_pkg(&self, pkg_name: &str) {
        let mut command = std::process::Command::new(&self.path);
        if self.sdk_root.is_some() {
            command.arg(format!(
                "--sdk_root={}",
                self.sdk_root.as_ref().unwrap().display()
            ));
        };

        let mut child = command.arg(pkg_name).stdin(Stdio::piped()).spawn().unwrap();
        write!(child.stdin.as_mut().unwrap(), "y\ny\n").unwrap();
        child.wait().unwrap();
    }
}

impl AvdManager {
    pub fn new(path: &PathBuf) -> AvdManager {
        return AvdManager { path: path.clone() };
    }

    pub fn register_device(&self, config: &EmulatorCfg) {
        let mut command = std::process::Command::new(&self.path);

        command.arg("create").arg("avd");
        if config.device_dir.is_some() {
            command.arg("-p").arg(config.device_dir.as_ref().unwrap());
        };

        command
            .arg("-n")
            .arg(&config.name)
            .arg("-d")
            .arg("pixel_xl")
            .arg("-k")
            .arg(&config.system_ver)
            .arg("-f");

        let process = command.spawn();
        process.unwrap().wait().unwrap();
    }

    pub fn find_registered_device(&self, avd_name: &str) -> Option<String> {
        let command = std::process::Command::new(&self.path)
            .arg("list")
            .arg("avd")
            .output();

        if command.is_err() {
            println!(
                "Failed to check fo the existing device: {}",
                command.unwrap_err()
            );
            return None;
        };

        let mut name_found = false;
        for line in command.unwrap().stdout.lines() {
            let str_line = line.unwrap();

            if !name_found {
                if str_line.trim().starts_with("Name") {
                    let current_avd_name = str_line.split(":").last().unwrap().trim();
                    if current_avd_name == avd_name {
                        name_found = true;
                    }
                };
            } else {
                if str_line.trim().starts_with("Path") {
                    let avd_path = str_line.split(":").last().unwrap().trim();
                    return Some(String::from(avd_path));
                }
            }
        }
        return None;
    }
}
