use std::{
    borrow::Borrow,
    fs,
    io::{Cursor, Write},
};

use quick_xml::{events::Event, Reader, Writer};

use super::CliAction;

pub struct ApkPatchNetConfigAction {}

const NET_SECURITY_CONFIG: &str = r#"
<?xml version="1.0" encoding="utf-8"?>
<network-security-config>
    <base-config>
        <trust-anchors>
        <certificates src="system" />
        <certificates src="user"/>
    </trust-anchors>
    </base-config>
</network-security-config>
"#;

impl CliAction for ApkPatchNetConfigAction {
    fn get_name(&self) -> &str {
        return "Patch APK to trust user SSL certificates.";
    }

    fn execute(&mut self, ctx: &mut crate::context::Context) {
        let network_config_path = &ctx.paths.dirs.apk_unpack_dir.join("res").join("xml");
        fs::create_dir_all(network_config_path).unwrap();
        let network_config_path = network_config_path.join("network_security_config.xml");
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&network_config_path)
            .unwrap();
        match file.write_all(&NET_SECURITY_CONFIG.trim().as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                println!(
                    "Failed to write to {}. {}",
                    network_config_path.display(),
                    err
                );
            }
        };

        println!("Network configuration successfully patched.");

        let mut reader = match Reader::from_file(&ctx.paths.apk.android_manifest) {
            Ok(res) => res,
            Err(err) => {
                println!(
                    "Failed to ready read {}. {}",
                    ctx.paths.apk.android_manifest.display(),
                    err
                );
                return;
            }
        };

        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref elem)) if elem.name() == b"application" => {
                    let mut new_element = elem.borrow().clone();
                    let has_config = new_element
                        .attributes()
                        .any(|attr| attr.unwrap().key == b"android:networkSecurityConfig");
                    if has_config {
                        println!("AndroidManifest.xml already has network security config attribute.");
                        return;
                    }

                    new_element.push_attribute((
                        "android:networkSecurityConfig",
                        "@xml/network_security_config",
                    ));
                    writer.write_event(Event::Start(new_element)).unwrap();
                }
                Ok(Event::Eof) => break,
                Ok(event) => writer.write_event(event).unwrap(),
                Err(err) => {
                    println!("Failed to parse manifest XML: {}", err);
                    return;
                }
            }
        }

        let mut manifest = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&ctx.paths.apk.android_manifest)
            .unwrap();
        manifest
            .write_all(&writer.into_inner().into_inner())
            .unwrap();
        println!("AndroidManifest.xml was updated to trust user certificates.");
    }

    fn can_execute(&self, ctx: &crate::context::Context) -> bool {
        if ctx.paths.apk.android_manifest.exists() {
            return true;
        }
        return false;
    }
}
