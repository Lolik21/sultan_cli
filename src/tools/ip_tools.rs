use std::{process::{Command}, str::FromStr};

pub struct IpTools {}

impl IpTools {
    pub fn get_host_ip() -> String {
        return IpTools::get_ip_windows();
    }

    fn get_ip_windows() -> String {
        let output = Command::new("cmd")
            .arg("/C")
            .arg("ping -4 -n 1 %ComputerName%")
            .output().unwrap();
        
        let string = String::from_utf8(output.stdout).unwrap();
        let ip_start = string.find("[").unwrap();
        let ip_end = string.find("]").unwrap();

        return String::from_str(&string[ip_start+1..ip_end]).unwrap();
    }
}
