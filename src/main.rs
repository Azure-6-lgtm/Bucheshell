mod strap;
mod handler;
mod utils;
use colored::Colorize;

fn main() {
    println!("---------------------------------------");
    println!("{}","BUCHESHELL ALPHA 0.5 TESTING ENVIORMENT".red());
    println!("{}","THIS IS FOR TESTING PURPOSES ONLY".red());
    println!("{}","ALPHA RELEASES ARE KNOWN TO BE UNSTABLE".red());
    println!("{}","BE CAUTIOUS OF BUGS".red());
    println!("{}","----------------------------------------");
    strap::init::init();
}

