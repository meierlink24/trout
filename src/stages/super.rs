/*
* Lists disks using lsblk and confirms user selection
  - Returns: selected disk path (e.g., /dev/sda)
*/


use std::process::Command;
use std::io::{self, Write};

pub fn select_disk() -> String {
        println!                                                              ("Available disks:");
        Command::new("sh")
                .arg("-c")
                .arg("lsblk -d -o NAME,SIZE,MODEL | grep -E '^sd|^nvme'")
                .status()
                .expect("Failed to run lsblk");

        print!                                                                ("Enter the disk to wipe (e.g., /dev/sda): ");
        io::stdout().flush().unwrap();

        let mut disk                     =                                   String::new();
        io::stdin().read_line(&mut disk).unwrap();

        disk.trim().to_string()
}


/*
* Wipes all partition signatures and data on the given disk
*/

pub fn wipe_disk(disk: &str) {
        println!                                                              ("Wiping all data on {} ...", disk);
        let cmd                          =                                   format!("sgdisk --zap-all {}", disk);
        Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status()
                .expect("Failed to wipe disk");

        let cmd2                         =                                   format!("wipefs -a {}", disk);
        Command::new("sh")
                .arg("-c")
                .arg(cmd2)
                .status()
                .expect("Failed to clean filesystem signatures");
}
