/*
* Installer logic selector
  - Based on user boot setup: Clean Wipe or Dual Boot
*/

use crate::stages::{cleanwipe, dualboot};
use std::io::Write;


pub fn begin_install() {
        println!                                                              ("How do you want to proceed with disk setup?");
        print!                                                                ("Enter 1 (Clean Wipe) or Enter 2 (Dual Boot): ");
        std::io::stdout().flush().unwrap();

        let mut boot_choice              =                                   String::new();
        std::io::stdin().read_line(&mut boot_choice).unwrap();
        let boot_choice                  =                                   boot_choice.trim().parse::<u8>().unwrap_or(0);

        if boot_choice == 1                                                 { cleanwipe::clean_wipe(); }
        else if boot_choice == 2                                            { dualboot::dual_boot_setup(); }
        else                                                                 { println!("Invalid input. Aborting."); }
}
