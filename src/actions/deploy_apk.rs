use super::CliAction;

use crate::tools::adb::AdbTool;
use crate::tools::apk_signer::ApkSignerV1;
use crate::tools::apk_tool::ApkTool;
use crate::tools::corona_archiver::CoronaArchiver;

pub struct DeployApkAction {}

impl CliAction for DeployApkAction {
    fn get_name(&self) -> &str {
        return "Deploy decompiled APK package.";
    }

    fn execute(&mut self, ctx: &mut crate::context::Context) {
        let corona_archiver = CoronaArchiver::new(&ctx.paths.tools.c_archiver_path);
        corona_archiver.pack(&ctx.paths.dirs.lua_unpack_dir, &ctx.paths.apk.lua_resources);

        let apk_tool = ApkTool::new(&ctx.paths.tools.apktool_path);
        apk_tool.compile(&ctx.paths.dirs.apk_unpack_dir, &ctx.paths.apk_dist);

        let apk_signer = ApkSignerV1::new(&ctx.config.apk_key, &ctx.paths.sign_store);
        apk_signer.sign_apk_archive(&ctx.paths.apk_dist);

        let adb_tool = AdbTool::new(&ctx.paths.tools.adb_tool);
        adb_tool.install(&ctx.paths.apk_dist);
    }

    fn can_execute(&self, ctx: &crate::context::Context) -> bool {
        if ctx.paths.dirs.apk_unpack_dir.exists() {
            return true;
        }
        return false;
    }
}
