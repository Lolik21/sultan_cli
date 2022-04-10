use crate::config::Configuration;

use std::{env, fs};
use std::path::PathBuf;

pub struct Directories {
    pub working_dir: PathBuf,
    pub lua_unpack_dir: PathBuf,
    pub apk_download_dir: PathBuf,
    pub apk_unpack_dir: PathBuf,
    pub tools_dir: PathBuf,
    pub android_tools_dir: PathBuf,
    pub android_sdk_dir: PathBuf,
}

pub struct CmdTools {
    pub sdk_manager_path: PathBuf,
    pub avd_manager_path: PathBuf,
}

pub struct Tools {
    pub apk_cmd: CmdTools,

    pub c_archiver_path: PathBuf,
    pub apktool_path: PathBuf,
    pub g_play_path: PathBuf,
    pub adb_tool: PathBuf,
    pub proxy_dump: PathBuf,
}

pub struct Emulator {
    pub cli_path: PathBuf,
}

pub struct ApkPaths {
    pub lua_resources: PathBuf,
    pub android_manifest: PathBuf,
}

pub struct Paths {
    pub dirs: Directories,
    pub emulator: Emulator,
    pub tools: Tools,

    pub apk_dist: PathBuf,
    pub sign_store: PathBuf,
    pub trace_dir: PathBuf,
    pub apk: ApkPaths,
}

pub struct Context {
    pub paths: Paths,
    pub config: Configuration,
}

impl Context {
    pub fn initialize() -> Context {
        let working_dir = match env::current_dir() {
            Ok(val) => val,
            Err(err) => {
                panic!(
                    "Error occurred while trying to get working directory: {}",
                    err
                );
            }
        };

        let config: Configuration = Configuration::read_from(working_dir.join("config.json"));

        let wd = working_dir.join("wd");
        let apk_download_dir = wd.join("apk");
        let apk_unpack_dir = wd.join("unpack");
        let lua_unpack_dir = wd.join("lua_unpacked");
        let tools_dir = wd.join("tools");
        let sign_store = wd.join("sign.keystore");
        let apk_dist = wd.join("dist").join("install.apk");

        let android_sdk_dir = tools_dir.join("android-sdk");
        let android_tools_dir = tools_dir.join("cmdline-tools");
        let sdk_manager_path = android_tools_dir.join("bin").join("sdkmanager.bat");
        let avd_manager_path = android_tools_dir.join("bin").join("avdmanager.bat");
        let apktool_path = tools_dir.join("apktool.jar");
        let c_archiver_path = tools_dir.join("c_archiver");
        let g_play_path = tools_dir.join("g_play");
        let platform_tools = android_sdk_dir.join("platform-tools");
        let adb_tool = platform_tools.join("adb.exe");
        let lua_resources = apk_unpack_dir.join("assets").join("resource.car");
        let android_manifest = apk_unpack_dir.join("AndroidManifest.xml");
        let emulator_path = android_sdk_dir.join("emulator").join("emulator.exe");
        let trace_dir = wd.join("trace");
        let proxy_dump = tools_dir.join("mitmdump.exe");

        fs::create_dir_all(&working_dir).unwrap();

        let context = Context {
            paths: Paths {
                dirs: Directories {
                    working_dir,
                    android_sdk_dir,
                    android_tools_dir,
                    apk_download_dir,
                    apk_unpack_dir,
                    lua_unpack_dir,
                    tools_dir,
                },
                emulator: Emulator {
                    cli_path: emulator_path
                },
                tools: Tools {
                    adb_tool,
                    apktool_path,
                    c_archiver_path,
                    g_play_path,
                    proxy_dump,
                    apk_cmd: CmdTools {
                        avd_manager_path,
                        sdk_manager_path
                    }
                },
                apk: ApkPaths {
                    lua_resources,
                    android_manifest
                },
                apk_dist,
                sign_store,
                trace_dir
            },
            config,
        };
        return context;
    }
}
