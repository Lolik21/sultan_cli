use std::fs::{self, DirEntry};
use std::path::PathBuf;

use super::CliAction;
use crate::context::Context;
use crate::tools::file_downloader;
use crate::tools::local_cmdline::SdkManager;

pub struct DownloadToolsAction {}

impl CliAction for DownloadToolsAction {
    fn execute(&mut self, ctx: &mut Context) {
        fs::create_dir_all(&ctx.paths.dirs.tools_dir).unwrap();

        download_google_play(&ctx);
        download_corona_archive(&ctx.paths.tools.c_archiver_path);
        download_apk_tool(&ctx.paths.tools.apktool_path);
        download_http_proxy(&ctx);

        // TODO: Can be detected on the runtime
        download_android_tools(&ctx);

        let sdk_manager = SdkManager::new(
            &ctx.paths.tools.apk_cmd.sdk_manager_path,
            Some(ctx.paths.dirs.android_sdk_dir.clone()),
        );

        sdk_manager.download_pkg("platform-tools");
    }

    fn get_name(&self) -> &str {
        return "Download tools.";
    }

    fn can_execute(&self, _: &Context) -> bool {
        return true;
    }
}

fn download_http_proxy(ctx: &&mut Context) {
    if !ctx.paths.tools.proxy_dump.exists() {
        file_downloader::download_and_unzip(
            "https://snapshots.mitmproxy.org/8.0.0/mitmproxy-8.0.0-windows.zip",
            &ctx.paths.dirs.tools_dir,
            None,
        );
    }
}

fn download_android_tools(ctx: &Context) {
    if !ctx.paths.tools.apk_cmd.sdk_manager_path.exists() {
        file_downloader::download_and_unzip(
            "https://dl.google.com/android/repository/commandlinetools-win-8092744_latest.zip",
            &ctx.paths.dirs.tools_dir,
            None,
        );
    }
}

fn download_corona_archive(c_archiver_path: &PathBuf) {
    if !c_archiver_path.exists() {
        file_downloader::download_file(
            "https://github.com/0BuRner/corona-archiver/releases/download/1.1/corona-archiver.exe",
            &c_archiver_path,
        );
    }
}

fn download_google_play(ctx: &Context) {
    if !ctx.paths.tools.g_play_path.exists() {
        file_downloader::download_and_unzip(
            "https://github.com/89z/googleplay/releases/download/v1.7.0/googleplay-windows.zip",
            &ctx.paths.dirs.tools_dir,
            Some(&|item: &DirEntry| {
                if item.file_name() == "googleplay.exe" {
                    fs::rename(item.path(), &ctx.paths.tools.g_play_path).unwrap();
                }
            }),
        );
    }
}

fn download_apk_tool(apktool_path: &PathBuf) {
    if !apktool_path.exists() {
        file_downloader::download_file(
            "https://bitbucket.org/iBotPeaches/apktool/downloads/apktool_2.6.1.jar",
            &apktool_path,
        );
    }
}
