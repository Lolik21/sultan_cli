use super::CliAction;
use crate::tools::emulator::EmulatorCli;

pub struct StartEmulatorAction {}

impl CliAction for StartEmulatorAction {
    fn get_name(&self) -> &str {
        return "Start emulator.";
    }

    fn execute(&mut self, ctx: &mut crate::context::Context) {
        let emulator = EmulatorCli::new(&ctx.paths.emulator.cli_path);
        emulator.start(
            ctx.config.emulator.as_ref().unwrap(),
            &ctx.paths.dirs.android_sdk_dir,
        );
    }

    fn can_execute(&self, _: &crate::context::Context) -> bool {
        return true;
    }
}
