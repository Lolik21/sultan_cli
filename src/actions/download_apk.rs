use dialoguer::Confirm;

use super::CliAction;

use crate::tools::apk_downloader;

pub struct DownloadApkAction {}

impl CliAction for DownloadApkAction {
    fn get_name(&self) -> &str {
        return "Download game APK.";
    }

    fn execute(&mut self, ctx: &mut crate::context::Context) {
        if ctx.config.g_play.uname.is_empty() || ctx.config.g_play.passwd.is_empty() {
            println!("Google account name(email) must to be specified in configuration.");
            println!("Google account password must be specified in configuration.");
            println!("Use https://myaccount.google.com/apppasswords to generate password.");
            return;
        }

        let g_play = &ctx.paths.tools.g_play_path;
        println!("Generating google play Token file...");
        let apk_key = &ctx.config.apk_key;
        if !apk_downloader::generate_store_token(
            &ctx.config.g_play.uname,
            &ctx.config.g_play.passwd,
            &g_play,
        ) {
            return;
        }

        println!("Generating fake device information...");
        if !apk_downloader::generate_device_info(&g_play) {
            return;
        }

        let version_code = match apk_downloader::query_for_last_version(apk_key, &g_play) {
            Some(value) => value,
            None => return,
        };

        println!(
            "Last version available for {} is {}.",
            apk_key, version_code
        );

        let existing_package = ctx
            .paths
            .dirs
            .apk_download_dir
            .join(format!("{}-{}.apk", apk_key, version_code));

        if existing_package.exists() {
            if !Confirm::new()
                .with_prompt(format!(
                    "APK exists: {}. Do you wish to continue downloading?",
                    existing_package.display()
                ))
                .interact()
                .unwrap()
            {
                return;
            }
        }

        println!("Downloading {} APK package...", apk_key);
        if !apk_downloader::download_apk(
            apk_key,
            &version_code,
            g_play,
            &ctx.paths.dirs.apk_download_dir,
        ) {
            return;
        }

        println!("Downloading {} APK package finished.", apk_key);
    }

    fn can_execute(&self, ctx: &crate::context::Context) -> bool {
        if ctx.paths.tools.g_play_path.exists() {
            return true;
        }
        return false;
    }
}
