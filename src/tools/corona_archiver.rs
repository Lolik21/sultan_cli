use std::{fs, path::PathBuf};

pub struct CoronaArchiver {
    path: PathBuf,
}

impl CoronaArchiver {
    pub fn new(path: &PathBuf) -> CoronaArchiver {
        return CoronaArchiver { path: path.clone() };
    }

    pub fn extract(&self, from_file: &PathBuf, to_dir: &PathBuf) -> bool {
        fs::create_dir_all(&to_dir).unwrap();
        match std::process::Command::new(&self.path)
            .arg("-u")
            .arg(from_file)
            .arg(to_dir)
            .output()
        {
            Ok(_) => true,
            Err(err) => {
                println!("Error while unpacking lua: {}.", err);
                false
            }
        }
    }

    pub fn pack(&self, unpack_dir: &PathBuf, to_file: &PathBuf) -> bool {
        match std::process::Command::new(&self.path)
            .arg("-p")
            .arg(unpack_dir)
            .arg(to_file)
            .output()
        {
            Ok(_) => true,
            Err(err) => {
                println!("Error while packing lua: {}.", err);
                false
            }
        }
    }
}
