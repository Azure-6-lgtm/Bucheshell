mod strap;
mod handler;
mod utils;
use colored::Colorize;
mod logger;
use logger::buchelog;

fn main() {
    buchelog::init_logger(); // only call once

    buchelog::log_info("Bucheshell started succesfully");
    buchelog::log_warn("This is a warning example");
    buchelog::log_error("This is an error example");

    /*println!("---------------------------------------");
    println!("{}","BUCHESHELL RELEASE 2.1.0 ".red());
    println!("{}","BUCHESHELL IS NOT FULLY COMPLETE YET".red());
    println!("{}","RELEASES ARE KNOWN TO BE UNSTABLE".red());
    println!("{}","BE CAUTIOUS OF BUGS".red());
    println!("{}","----------------------------------------");
    */
    strap::init::init();
}

