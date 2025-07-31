/*
* Mounts partitions to /mnt
*/
use std::process::Command;

pub fn mount_partitions() {
        Command::new("sh")
                .arg("-c")
                .arg("mount /dev/sda2 /mnt")
                .status()
                .expect("Failed to mount root");

        Command::new("sh")
                .arg("-c")
                .arg("mkdir -p /mnt/boot && mount /dev/sda1 /mnt/boot")
                .status()
                .expect("Failed to mount boot");
}
