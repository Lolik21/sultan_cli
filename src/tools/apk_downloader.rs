use std::{fs, path::PathBuf};

pub fn download_apk(
    apk_key: &str,
    version_code: &str,
    g_play_path: &PathBuf,
    download_dir: &PathBuf,
) -> bool {
    fs::create_dir_all(&download_dir).unwrap();
    match std::process::Command::new(&g_play_path)
        .arg("-a")
        .arg(apk_key)
        .arg("-v")
        .arg(version_code)
        .current_dir(download_dir)
        .output()
    {
        Ok(_) => {}
        Err(err) => {
            println!("Failed to download APK: {}", err);
            return false;
        }
    };
    return true;
}

pub fn query_for_last_version(apk_key: &str, g_play_path: &PathBuf) -> Option<String> {
    let package_info = match std::process::Command::new(&g_play_path)
        .arg("-a")
        .arg(apk_key)
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            println!("Failed to retrieve package information: {}", err);
            return None;
        }
    };
    let mut version_code = String::new();
    let package_info = std::str::from_utf8(&package_info.stdout).unwrap();
    let strings = package_info.split("\n");
    for version_data in strings {
        if version_data.starts_with("VersionCode") {
            version_code = version_data
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .to_string();
        }
    }

    Some(version_code)
}

pub fn generate_device_info(g_play_path: &PathBuf) -> bool {
    match std::process::Command::new(&g_play_path).arg("-d").output() {
        Ok(_) => {}
        Err(err) => {
            println!("Failed to create fake device ID: {}", err);
            return false;
        }
    }
    return true;
}

pub fn generate_store_token(uname: &str, passwd: &str, g_play_path: &PathBuf) -> bool {
    match std::process::Command::new(&g_play_path)
        .arg("-e")
        .arg(uname)
        .arg("-p")
        .arg(passwd)
        .output()
    {
        Ok(_) => {}
        Err(err) => {
            println!("Failed to create token file: {}", err);
            return false;
        }
    }
    return true;
}
