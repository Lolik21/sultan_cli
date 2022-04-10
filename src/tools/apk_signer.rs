use std::path::PathBuf;

pub struct ApkSignerV1 {
    apk_key: String,
    sign_store_path: PathBuf,
}

impl ApkSignerV1 {
    pub fn new(apk_key: &String, sign_store_path: &PathBuf) -> ApkSignerV1 {
        return ApkSignerV1 {
            apk_key: apk_key.clone(),
            sign_store_path: sign_store_path.clone(),
        }
    }

    pub fn sign_apk_archive(&self, apk_archive_path: &PathBuf) {
        self.generate_signature();
        println!("Signing {} APK package.", apk_archive_path.display());
        let mut sign_child = std::process::Command::new("jarsigner")
            .arg("-sigalg")
            .arg("SHA1withRSA")
            .arg("-digestalg")
            .arg("SHA1")
            .arg("-keystore")
            .arg(&self.sign_store_path)
            .arg(&apk_archive_path)
            .arg(&self.apk_key)
            .arg("-storepass")
            .arg("passwd")
            .spawn()
            .unwrap();
        sign_child.wait().unwrap();
    }

    fn generate_signature(&self) {
        if self.sign_store_path.exists() {
            return;
        }

        let mut sign_child = std::process::Command::new("keytool")
            .arg("-genkey")
            .arg("-v")
            .arg("-keystore")
            .arg(&self.sign_store_path)
            .arg("-alias")
            .arg(&self.apk_key)
            .arg("-keyalg")
            .arg("RSA")
            .arg("-storepass")
            .arg("passwd")
            .arg("-dname")
            .arg("cn=name,ou=group,o=company,c=country")
            .arg("-keysize")
            .arg("2048")
            .arg("-validity")
            .arg("365")
            .spawn()
            .unwrap();
        sign_child.wait().unwrap();
    }
}
