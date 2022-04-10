use crate::context::Context;

mod download_emulator;
mod apk_patch_net_cfg;
mod download_apk;
mod download_tools;
mod decompile_apk;
mod deploy_apk;
mod exit;
mod trace;
mod start_emulator;

pub trait CliAction {
    fn get_name(&self) -> &str;
    fn execute(&mut self, ctx: &mut Context);
    fn can_execute(&self, ctx: &Context) -> bool;
}

pub fn build_actions_que() -> Vec<Box<dyn CliAction>> {
    return vec![
        Box::new(download_tools::DownloadToolsAction {}),
        Box::new(download_apk::DownloadApkAction {}),
        Box::new(decompile_apk::DecompileApkAction {}),
        Box::new(deploy_apk::DeployApkAction {}),
        Box::new(apk_patch_net_cfg::ApkPatchNetConfigAction {}),
        Box::new(download_emulator::DownloadEmulatorAction {}),
        Box::new(start_emulator::StartEmulatorAction {}),
        Box::new(trace::TraceAction::new()),
        Box::new(exit::ExitProgramAction {}),
    ];
}
