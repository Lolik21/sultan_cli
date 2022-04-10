use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::Stdio,
};

use crate::{config::EmulatorCfg, tools::ip_tools::IpTools};

pub struct EmulatorCli {
    path: PathBuf,
}

impl EmulatorCli {
    pub fn new(path: &PathBuf) -> EmulatorCli {
        return EmulatorCli { path: path.clone() };
    }

    pub fn start(&self, config: &EmulatorCfg, sdk_path: &PathBuf) {
        let mut image_root = sdk_path.clone();

        config
            .system_ver
            .split(";")
            .for_each(|item| image_root.push(item));

        let mut child = std::process::Command::new(&self.path);
        child
            .arg("-avd")
            .arg(&config.name)
            .arg("-sysdir")
            .arg(image_root)
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .stdout(Stdio::piped());

        if let Some(http_proxy) = config.http_proxy {
            if http_proxy {
                let ip_address = IpTools::get_host_ip();
                child.arg("-http-proxy").arg(format!("{}:8080", ip_address));
            }
        }

        println!("Starting emulator...");
        let mut child = child.spawn().unwrap();
        let mut line_buffer = String::new();
        let mut reader = BufReader::new(child.stdout.take().unwrap());
        loop {
            if let Some(_) = child.try_wait().unwrap() {
                return;
            }

            reader.read_line(&mut line_buffer).unwrap();
            if line_buffer.contains("GRPC") {
                return;
            }
        }
    }
}
