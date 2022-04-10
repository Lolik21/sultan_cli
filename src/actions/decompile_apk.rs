use std::fs;

use dialoguer::Select;

use super::CliAction;
use crate::context::Context;
use crate::tools::apk_tool::ApkTool;
use crate::tools::corona_archiver::CoronaArchiver;

pub struct DecompileApkAction {}

impl CliAction for DecompileApkAction {
    fn get_name(&self) -> &str {
        return "Decompile available APK.";
    }

    fn execute(&mut self, ctx: &mut crate::context::Context) {
        let apk_name = match detect_downloaded_apk(ctx) {
            Ok(res) => res,
            Err(err) => {
                println!("Failed to detect installed versions of APK. {}", err);
                return;
            }
        };

        let apk_path = ctx.paths.dirs.apk_download_dir.join(apk_name);
        let apk_tool = ApkTool::new(&ctx.paths.tools.apktool_path);
        let corona_arch = CoronaArchiver::new(&ctx.paths.tools.c_archiver_path);

        if !apk_tool.decompile(&apk_path, &ctx.paths.dirs.apk_unpack_dir) {
            return;
        }

        corona_arch.extract(&ctx.paths.apk.lua_resources, &ctx.paths.dirs.lua_unpack_dir);
    }

    fn can_execute(&self, ctx: &crate::context::Context) -> bool {
        if ctx.paths.dirs.apk_download_dir.exists() && ctx.paths.dirs.tools_dir.exists() {
            return true;
        }
        return false;
    }
}

fn detect_downloaded_apk(ctx: &Context) -> Result<String, String> {
    let read_dir = fs::read_dir(&ctx.paths.dirs.apk_download_dir).unwrap();
    let files: Vec<_> = read_dir
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    match files.len() {
        0 => {
            return Err(format!(
                "No APK files detected in {}.",
                ctx.paths.dirs.apk_download_dir.display()
            ));
        }
        1 => return Ok(files[0].to_string()),
        _other => {
            let option = match Select::new()
                .with_prompt("Please select APK file:")
                .items(&files)
                .interact()
            {
                Ok(res) => res,
                Err(_) => {
                    return Err("Failed to select APK from the list of packages.".to_string());
                }
            };
            return Ok(files[option].to_string());
        }
    };
}
