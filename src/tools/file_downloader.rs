use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::PathBuf;

pub fn download_file(url: &str, path_to_save: &PathBuf) {
    match download_file_internal(url, path_to_save) {
        Ok(_) => {}
        Err(err) => {
            println!("Error downloading file {}: {}", url, err);
            return;
        }
    };
}

pub fn download_and_unzip(url: &str, path_to_unzip: &PathBuf, filter: Option<&dyn Fn(&DirEntry)>) {
    match download_and_unzip_internal(url, path_to_unzip, filter) {
        Ok(_) => {}
        Err(err) => {
            println!("Error while downloading {}:{}", url, err);
            return;
        }
    };
}

fn download_file_internal(url: &str, path_to_save: &PathBuf) -> Result<(), String> {
    println!("Downloading {}", url);
    let file_request = match reqwest::blocking::get(url) {
        Ok(req) => req,
        Err(err) => return Err(err.to_string()),
    };

    let bytes_buffer = match file_request.bytes() {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.to_string()),
    };

    match File::create(path_to_save) {
        Ok(mut file) => match file.write_all(&bytes_buffer) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err.to_string()),
        },
        Err(err) => return Err(err.to_string()),
    }
}

fn download_and_unzip_internal(
    url: &str,
    path_to_unzip: &PathBuf,
    filter: Option<&dyn Fn(&DirEntry)>,
) -> Result<(), String> {
    fs::create_dir_all(&path_to_unzip).unwrap();
    let temp_file = path_to_unzip.join("temp.art");
    download_file(url, &temp_file);

    let zip_file = std::fs::File::open(&temp_file).unwrap();
    match zip::ZipArchive::new(zip_file)
        .unwrap()
        .extract(&path_to_unzip)
    {
        Ok(_) => {}
        Err(res) => {
            fs::remove_file(temp_file).unwrap();
            return Err(format!("Unable to unzip archive: {}", res));
        }
    }

    fs::remove_file(temp_file).unwrap();

    if filter.is_some() {
        for dir_entry in fs::read_dir(&path_to_unzip).unwrap() {
            if dir_entry.is_ok() {
                filter.unwrap()(&dir_entry.unwrap());
            }
        }
    }

    return Ok(());
}
