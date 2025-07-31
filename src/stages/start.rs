/*
* Really just entry point of User Installation experience
  - Handles choice to proceed or abort installation
*/


use std::io::{self, Write};
use crate::installer;

pub fn start() {
        println!                                                              ("Hi, you've started installation of Arch on your device.");
        println!                                                              ("Would you like to proceed?");
        print!                                                                ("Yes(enter 1) or No(enter 0): ");
        io::stdout().flush().unwrap();

        let mut s                         =                                  String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s                             =                                  s.trim().parse::<u8>().unwrap_or(0);

        if s == 1                                                            { installer::begin_install(); }
        else                                                                  { information_window(); }
}



/*
* User chose not to proceed
  - Display some info or tips
*/

fn information_window() {
        println!                                                              ("Exiting installer. If you're unsure, visit archlinux.org or consult the documentation.");
}
