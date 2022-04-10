use crate::context::Context;

use super::CliAction;

pub struct ExitProgramAction {}

impl CliAction for ExitProgramAction {
    fn get_name(&self) -> &str {
        return "Exit.";
    }

    fn execute(&mut self, _: &mut Context) {
        std::process::exit(0);
    }

    fn can_execute(&self, _: &Context) -> bool {
        return true;
    }
}
