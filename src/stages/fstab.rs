/*
* Generates fstab entries into new system
*/
use std::process::Command;

pub fn generate_fstab() {
        Command::new("sh")
                .arg("-c")
                .arg("genfstab -U /mnt >> /mnt/etc/fstab")
                .status()
                .expect("Failed to generate fstab");
}
