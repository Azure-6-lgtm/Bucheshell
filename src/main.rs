mod handler;
mod strap;
mod utils;
use colored::Colorize;
mod logger;
use logger::buchelog;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        handler::check::decipher_args(args);
    } else {
    }
    buchelog::init_logger(); // only call once

    buchelog::log_info("Bucheshell started succesfully");
    buchelog::log_warn("This is a warning example");
    buchelog::log_error("This is an error example");
    strap::init::init();
}
