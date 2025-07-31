/*
* Enters chroot to configure system
*/
use std::process::Command;

pub fn enter_chroot() {
        Command::new("sh")
                .arg("-c")
                .arg("arch-chroot /mnt")
                .status()
                .expect("arch-chroot failed");
}
