/*
* Verifies internet connection via ping
*/
use std::process::Command;

pub fn verify_internet() {
        let result = Command::new("sh")
                .arg("-c")
                .arg("ping -c 1 archlinux.org")
                .status()
                .expect("Failed to check internet");

        if !result.success() {
                println!                                                      ("Internet not working. Please connect manually.");
                std::process::exit(1);
        }
}
