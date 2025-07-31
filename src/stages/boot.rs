/*
* Installs base Arch packages using pacstrap
*/
use std::process::Command;

pub fn install_base() {
        Command::new("sh")
                .arg("-c")
                .arg("pacstrap /mnt base linux linux-firmware")
                .status()
                .expect("pacstrap failed");
}
