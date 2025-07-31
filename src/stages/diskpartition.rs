/*
* Partitions the disk for clean Arch install
  - Creates EFI, root (and optionally swap)
*/
use std::process::Command;



pub fn partition_disk(disk: &str) {
        let cmd = format!("
                parted -s {} mklabel gpt &&
                parted -s {} mkpart ESP fat32 1MiB 513MiB &&
                parted -s {} set 1 boot on &&
                parted -s {} mkpart primary ext4 513MiB 100%%",
                disk, disk, disk, disk);

        Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status()
                .expect("Partitioning failed");
}
