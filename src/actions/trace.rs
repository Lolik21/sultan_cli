use std::{
    fs,
    path::PathBuf,
    process::Child,
    thread,
    time::{Duration, UNIX_EPOCH},
};

use super::CliAction;
use crate::{
    context::Context,
    tools::{adb::AdbTool, emulator::EmulatorCli, http_proxy::HttpProxyTool},
};

pub struct TraceAction {
    is_active: bool,
    log_child: Option<Child>,
}

impl Drop for TraceAction {
    fn drop(&mut self) {
        if let Some(mut log_child) = self.log_child.take() {
            log_child.kill().unwrap();
        }
    }
}

impl TraceAction {
    pub fn new() -> TraceAction {
        return TraceAction {
            is_active: false,
            log_child: None,
        };
    }

    fn start_log_session(&mut self, ctx: &Context, trace_dir: &PathBuf) {
        let log_to = trace_dir.join(format!("{}_logcat.log", ctx.config.apk_key));
        let adb_tool = AdbTool::new(&ctx.paths.tools.adb_tool);
        self.log_child = adb_tool.start_log(&ctx.config.apk_key, &log_to);
    }

    fn start_net_trace_session(&mut self, ctx: &mut Context, trace_dir: &PathBuf) {
        let log_to = trace_dir.join(format!("{}_network.net", ctx.config.apk_key));
        let proxy_tool = HttpProxyTool::new(&ctx.paths.tools.proxy_dump);
        proxy_tool.start(&log_to);
    }

    fn create_trace_dir(ctx: &Context) -> std::path::PathBuf {
        let seconds_elapsed = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let current_trace_dir = ctx
            .paths
            .trace_dir
            .join(format!("Session_{}", seconds_elapsed));
        fs::create_dir_all(&current_trace_dir).unwrap();
        current_trace_dir
    }

    fn start_emulator_with_app(&self, ctx: &mut Context) {
        let adb_tool = AdbTool::new(&ctx.paths.tools.adb_tool);

        let devices = adb_tool.get_devices();
        if !devices.is_empty() {
            adb_tool.kill_emulator();
        }

        let emulator = EmulatorCli::new(&ctx.paths.emulator.cli_path);
        emulator.start(
            ctx.config.emulator.as_ref().unwrap(),
            &ctx.paths.dirs.android_sdk_dir,
        );

        thread::sleep(Duration::from_millis(3000));
        adb_tool.restart_application(&ctx.config.apk_key);
    }

    fn start_trace_sessions(&mut self, ctx: &mut Context) {
        let current_trace_dir = TraceAction::create_trace_dir(ctx);
        self.start_net_trace_session(ctx, &current_trace_dir);
        thread::sleep(Duration::from_millis(1000));
        self.start_emulator_with_app(ctx);
        thread::sleep(Duration::from_millis(1000));
        self.start_log_session(ctx, &current_trace_dir);
        self.is_active = true;
    }

    fn stop_trace_sessions(&mut self, ctx: &Context) {
        if let Some(mut log_child) = self.log_child.take() {
            if let Err(err) = log_child.kill() {
                println!("Failed to kill log child: {}", err);
            }
        }

        HttpProxyTool::stop();
        let adb_tool = AdbTool::new(&ctx.paths.tools.adb_tool);
        adb_tool.kill_emulator();

        self.is_active = false;
    }
}

impl CliAction for TraceAction {
    fn get_name(&self) -> &str {
        if self.is_active {
            return "Stop trace session.";
        }
        return "Start trace session.";
    }

    fn execute(&mut self, ctx: &mut Context) {
        if self.is_active {
            self.stop_trace_sessions(ctx);
        } else {
            self.start_trace_sessions(ctx);
        }
    }

    fn can_execute(&self, ctx: &crate::context::Context) -> bool {
        if ctx.paths.dirs.tools_dir.exists() {
            return true;
        }
        return false;
    }
}
