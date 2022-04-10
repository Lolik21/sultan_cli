use std::{path::PathBuf, process::Command};

pub struct ApkTool {
    path: PathBuf,
}

impl ApkTool {
    pub fn new(path: &PathBuf) -> ApkTool {
        return ApkTool { path: path.clone() };
    }

    pub fn decompile(&self, apk_path: &PathBuf, unpack_dir: &PathBuf) -> bool {
        println!("Starting to decompile APK archive {}", apk_path.display());
        let mut child = match Command::new("java")
            .arg("-jar")
            .arg(&self.path)
            .arg("d")
            .arg(&apk_path)
            .arg("-o")
            .arg(&unpack_dir)
            .spawn()
        {
            Ok(res) => res,
            Err(err) => {
                println!("Error while starting apktool: {}", err);
                return false;
            }
        };
        match child.wait() {
            Ok(_) => {}
            Err(err) => {
                println!("Error while running apktool: {}", err);
                return false;
            }
        }
        println!("APK archive {} successfully decompiled", apk_path.display());
        return true;
    }

    pub fn compile(&self, unpack_dir: &PathBuf, to_apk: &PathBuf) {
        let mut child = std::process::Command::new("java")
            .arg("-jar")
            .arg(&self.path)
            .arg("b")
            .arg(unpack_dir)
            .arg("-o")
            .arg(to_apk)
            .spawn()
            .unwrap();
        child.wait().unwrap();
    }
}
