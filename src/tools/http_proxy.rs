use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct HttpProxyTool {
    path: PathBuf,
}

impl HttpProxyTool {
    pub fn new(path: &PathBuf) -> HttpProxyTool {
        return HttpProxyTool { path: path.clone() };
    }

    pub fn start(&self, log_to: &PathBuf) {
        let mut command = Command::new(&self.path);
        command.arg("-w").arg(log_to);
        command
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        command.spawn().unwrap();
    }

    pub fn stop() {
        Command::new("cmd").arg("/C").arg("taskkill /IM mitmdump.exe /F").output().unwrap();
    }
}
