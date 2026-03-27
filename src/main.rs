mod strap;
mod handler;
mod utils;
use colored::Colorize;

fn main() {
    println!("---------------------------------------");
    println!("{}","BUCHESHELL RELEASE 1.0 ENVIORMENT".red());
    println!("{}","AS SHELL IS NOT FULLY COMPLETE YET,IT CAN BE BUGGY".red());
    println!("{}","RELEASES ARE KNOWN TO BE UNSTABLE".red());
    println!("{}","BE CAUTIOUS OF BUGS".red());
    println!("{}","----------------------------------------");
    strap::init::init();
}

