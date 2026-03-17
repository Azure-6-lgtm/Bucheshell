use std::path::Path;
use colored::*;
use std::process::Command;
use crate::utils::run;

const USERPATH: &str = "/usr/bin/";

const TERMUXPATH: &str = "/data/data/com.termux/files/usr/bin/";

const SHELLPATH: &str = "/bin/";

pub fn checkcmd(input_command:&str) {
    if input_command.is_empty() {
        return;
    }
    let mut parts = input_command.split_whitespace();
    let cmd = parts.next().unwrap();
    let cmdargs: Vec<&str> = parts.collect();
    let termuxpath = format!("{}{}",TERMUXPATH,cmd);
    let userpath = format!("{}{}",USERPATH,cmd);
    let shellpath = format!("{}{}",SHELLPATH,cmd);
    if Path::new(&termuxpath).exists() {
        Command::new(&termuxpath)
            .args(&cmdargs)
            .status()
            .expect("Bucheshell failed to run");
    } else if Path::new(&userpath).exists() {
        Command::new(&userpath)
            .args(&cmdargs)
            .status()
            .expect("Bucheshell failed to run");
    } else if Path::new(&shellpath).exists() {
        Command::new(&shellpath)
            .args(&cmdargs)
            .status()
            .expect("Bucheshell failed to run");
    } else {
        checkutils(&cmd,&cmdargs);
    }
}
pub fn checkutils(utilcmd: &str,cmdargs: &Vec<&str>) {
    match utilcmd {
        "cd" => {
            run::builtin_cd(&cmdargs)
        }
        "ver" => {
            run::bshversion();
        }
        "help" => {
            run::help();
        }
        "about" => {
            run::aboutbsh();
        }
        "exit" => {
            run::exit();
        }
        _ => {
            println!("{}","Not available in bsh library,Maybe it is not installed?".yellow());
        }
    };
}
