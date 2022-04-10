use std::{
    fs,
    io::{BufRead, Read, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
};

pub struct AdbTool {
    path: PathBuf,
}

impl AdbTool {
    pub fn new(path: &PathBuf) -> AdbTool {
        return AdbTool { path: path.clone() };
    }

    pub fn install(&self, apk_path: &PathBuf) {
        let mut child = Command::new(&self.path)
            .arg("install")
            .arg("-r")
            .arg(apk_path)
            .spawn()
            .unwrap();

        child.wait().unwrap();
    }

    pub fn start_log(&self, app_name: &str, log_to: &PathBuf) -> Option<Child> {
        let pid = self.find_pid(app_name);
        if pid.is_none() {
            println!(
                "Cannot find application {} process id. Recording all logs.",
                app_name
            );
        }

        Command::new(&self.path)
            .arg("logcat")
            .arg("-c")
            .output()
            .unwrap();

        let mut command = Command::new(&self.path);
        command
            .arg("logcat")
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .stdout(Stdio::piped());

        if let Some(pid) = pid {
            command.arg(format!("--pid={}", pid));
        }

        let mut child = command.spawn().unwrap();

        let log_file_path = log_to.clone();
        let child_output = child.stdout.take();
        std::thread::spawn(move || {
            let mut buf = [0; 256];
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(log_file_path)
                .unwrap();
            let mut child_output = child_output.unwrap();

            loop {
                let count_read = child_output.read(&mut buf).unwrap();
                if count_read == 0 {
                    break;
                }

                let to_write_buff = &buf[..count_read];
                file.write(to_write_buff).unwrap();
            }
        });

        return Some(child);
    }

    pub fn get_devices(&self) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        let output = Command::new(&self.path).arg("devices").output().unwrap();
        for line in output.stdout.lines().skip(1) {
            let device = line.as_ref().unwrap().split("\t").next().unwrap().trim();
            if !device.is_empty() {
                results.push(device.to_string());
            }
        }

        return results;
    }

    pub fn find_pid(&self, app_name: &str) -> Option<String> {
        let output = Command::new(&self.path)
            .arg("shell")
            .arg("pidof")
            .arg("-s")
            .arg(app_name)
            .output()
            .unwrap();
        if output.stdout.is_empty() {
            return None;
        }
        return Some(String::from_utf8(output.stdout).unwrap().trim().to_string());
    }

    pub fn restart_application(&self, apk_key: &str) {
        Command::new(&self.path)
            .arg("shell")
            .arg("am")
            .arg("force-stop")
            .arg(apk_key)
            .output()
            .unwrap();
        Command::new(&self.path)
            .arg("shell")
            .arg("monkey")
            .arg("-p")
            .arg(apk_key)
            .arg("1")
            .output()
            .unwrap();
    }

    pub fn kill_emulator(&self) {
        Command::new(&self.path).arg("emu").arg("kill").output().unwrap();
    }
}
