use crate::utils::run;
use colored::*;
use std::path::Path;
use std::process::Command;

const USERPATH: &str = "/usr/bin/";

const TERMUXPATH: &str = "/data/data/com.termux/files/usr/bin/";

const SHELLPATH: &str = "/bin/";

pub fn checkcmd(input_command: &str) {
    if input_command.is_empty() {
        return;
    }
    let mut parts = input_command.split_whitespace();
    let cmd = parts.next().unwrap();
    let cmdargs: Vec<&str> = parts.collect();
    let termuxpath = format!("{}{}", TERMUXPATH, cmd);
    let userpath = format!("{}{}", USERPATH, cmd);
    let shellpath = format!("{}{}", SHELLPATH, cmd);
    if Path::new(&termuxpath).exists() && !checkforbuiltin(cmd) {
        Command::new(&termuxpath)
            .args(&cmdargs)
            .status()
            .expect("Bucheshell failed to run");
    } else if Path::new(&userpath).exists() && !checkforbuiltin(cmd) {
        Command::new(&userpath)
            .args(&cmdargs)
            .status()
            .expect("Bucheshell failed to run");
    } else if Path::new(&shellpath).exists() && !checkforbuiltin(cmd) {
        Command::new(&shellpath)
            .args(&cmdargs)
            .status()
            .expect("Bucheshell failed to run");
    } else {
        checkutils(&cmd, &cmdargs);
    }
}
pub fn checkutils(utilcmd: &str, cmdargs: &Vec<&str>) {
    match utilcmd {
        "cd" => run::builtin_cd(&cmdargs),
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
        "mkdir" => {
            run::mkdir(&cmdargs);
        }
        "rm" => {
            run::rm(&cmdargs);
        }
        "rmdir" => {
            run::rmdir(&cmdargs);
        }
        "touch" => {
            run::touch(&cmdargs);
        }
        "cp" => {
            run::cp(&cmdargs);
        }
        "ls" => {
            run::ls(&cmdargs);
        }
        "mv" => {
            run::mv(&cmdargs);
        }
        _ => {
            println!(
                "{}",
                "Not available in bsh library,Maybe it is not installed?".yellow()
            );
        }
    };
}
fn checkforbuiltin(cmd: &str) -> bool {
    if cmd == "mkdir"
        || cmd == "rm"
        || cmd == "touch"
        || cmd == "mv"
        || cmd == "cp"
        || cmd == "rmdir"
        || cmd == "ls"
    {
        true
    } else {
        false
    }
}
