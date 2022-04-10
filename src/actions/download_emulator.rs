use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use crate::tools::local_cmdline::{AvdManager, SdkManager};

use super::CliAction;

pub struct DownloadEmulatorAction {}

impl CliAction for DownloadEmulatorAction {
    fn get_name(&self) -> &str {
        return "Setup local emulator.";
    }

    fn execute(&mut self, ctx: &mut crate::context::Context) {
        let emulator_cfg = ctx.config.emulator.as_ref().unwrap();
        let manager = AvdManager::new(&ctx.paths.tools.apk_cmd.avd_manager_path);
        manager.register_device(emulator_cfg);
        let avd_dir = manager.find_registered_device(&emulator_cfg.name);

        let avd_dir = avd_dir.unwrap();
        let sdk_manager = SdkManager::new(
            &ctx.paths.tools.apk_cmd.sdk_manager_path,
            Some(ctx.paths.dirs.android_sdk_dir.clone()),
        );

        sdk_manager.download_pkg(&emulator_cfg.system_ver);
        sdk_manager.download_pkg("platform-tools");
        fs::create_dir_all(&ctx.paths.dirs.android_sdk_dir.join("platforms")).unwrap();

        let config_path = PathBuf::from(&avd_dir).join("config.ini");
        let mut cfg_file = fs::OpenOptions::new()
            .read(true)
            .open(&config_path)
            .unwrap();

        let mut read_buffer = String::new();
        cfg_file.read_to_string(&mut read_buffer).unwrap();
        let mut lines: Vec<&str> = read_buffer.split('\n').collect();
        lines.remove(lines.len() - 1);

        let mut cfg_file = fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&config_path)
            .unwrap();

        let replacement_map = HashMap::from([(
            "disk.dataPartition.size",
            emulator_cfg.sd_size.as_deref().unwrap_or("5000M"),
        )]);

        for line in lines {
            let mut split: Vec<&str> = line.split("=").collect();
            if replacement_map.contains_key(split[0]) {
                split[1] = replacement_map[split[0]];
            }

            cfg_file
                .write(format!("{}={}\n", split[0], split[1]).as_bytes())
                .unwrap();
        }

        println!(
            "Emulator device was created and configured under {}",
            config_path.display()
        );
    }

    fn can_execute(&self, ctx: &crate::context::Context) -> bool {
        if ctx.paths.dirs.android_tools_dir.exists() {
            return true;
        }

        return false;
    }
}
