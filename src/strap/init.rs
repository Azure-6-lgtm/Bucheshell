use crate::handler::check;
use crate::logger::buchelog::{log_error, log_info, log_warn};
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env;
use std::io::{self, Write};

fn get_display_dir() -> String {
    let dir = env::current_dir().unwrap();
    let home = env::var("HOME").unwrap();

    let path = dir.display().to_string();

    if path.starts_with(&home) {
        path.replacen(&home, "~", 1)
    } else {
        path
    }
}

pub fn init() {
    let mut rl = DefaultEditor::new().unwrap();
    log_info("Bootstrap started");
    //println!("Buche Shell 0.01 Testing enviorment");
    //println!("Made with love ");
    //println!("------------------------------------");
    loop {
        let cpath = get_display_dir();
        let prompt = format!("[{}]{}", cpath.cyan(), ">>".cyan());
        //print!("[{}] >>",get_display_dir());
        let input_command = match rl.readline(&prompt) {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                input
            }
            Err(ReadlineError::Interrupted) => {
                println!(); // Ctrl+C
                continue;
            }
            Err(ReadlineError::Eof) => {
                break; // Ctrl+D exits shell
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
        let input_command = input_command.trim();
        io::stdout().flush().unwrap();
        check::checkcmd(&input_command);
        log_info("Command passed to handler");
    }
}
