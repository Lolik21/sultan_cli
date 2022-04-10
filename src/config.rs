use serde_derive::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Serialize)]
pub struct GPlayCfg {
    pub uname: String,
    pub passwd: String,
}

#[derive(Deserialize, Serialize)]
pub struct EmulatorCfg {
    pub name: String,
    pub system_ver: String,
    pub device_dir: Option<String>,
    pub sd_size: Option<String>,
    pub http_proxy: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub apk_key: String,
    pub g_play: GPlayCfg,
    pub emulator: Option<EmulatorCfg>,
}

impl Configuration {
    pub fn read_from(path: PathBuf) -> Configuration {
        if !path.exists() {
            panic!("Configuration file 'config.json' is mandatory for application to work");
        }

        let config = fs::read_to_string(path).unwrap();
        return serde_json::from_str(&config).unwrap();
    }
}
