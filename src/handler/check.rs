//import necessary crates
use crate::logger::buchelog::{log_error, log_info, log_warn};
use crate::utils::run;
use colored::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio}; 
use crate::logger::buchelog::{log_info, log_warn, log_error};
//constants
const USERPATH: &str = "/usr/bin/";
const TERMUXPATH: &str = "/data/data/com.termux/files/usr/bin/";
const SHELLPATH: &str = "/bin/";

pub fn checkcmd(input_command: &str) {
    if input_command.is_empty() {
        log_warn("Input command empty. Returning");
        return;
    }

    // Handle output redirection
    let mut parts: Vec<&str> = input_command.split_whitespace().collect();
    let mut output_file: Option<&str> = None;

    if let Some(pos) = parts.iter().position(|&x| x == ">>") {
        if pos + 1 < parts.len() {
            output_file = Some(parts[pos + 1]);
            parts.truncate(pos); // keep only the command parts
        } else {
            println!("{}", "Syntax error: no file specified for >>".red());
            return;
        }
    }

    let cmd = parts[0];
    let cmdargs: Vec<&str> = parts[1..].to_vec();

    let termuxpath = format!("{}{}", TERMUXPATH, cmd);
    let userpath = format!("{}{}", USERPATH, cmd);
    let shellpath = format!("{}{}", SHELLPATH, cmd);

    let mut command_to_run = if Path::new(&termuxpath).exists() && !checkforbuiltin(cmd) {
        log_info(&format!("Ran command succesfully {}", termuxpath));
        Command::new(&termuxpath)
    } else if Path::new(&userpath).exists() && !checkforbuiltin(cmd) {
        log_info(&format!("Ran command succesfully {}", userpath));
        Command::new(&userpath)
    } else if Path::new(&shellpath).exists() && !checkforbuiltin(cmd) {
        log_info(&format!("Ran command succesfully {}", shellpath));
        Command::new(&shellpath)
    } else {
        checkutils(&cmd, &cmdargs);
        log_warn("Passing unknown command to utils checker");
        return;
    };

    command_to_run.args(&cmdargs);

    if let Some(file) = output_file {
        let file_handle = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file)
            .expect("Failed to open file for appending");
        command_to_run.stdout(Stdio::from(file_handle));
    }

    command_to_run.status().expect("Bucheshell failed to run");
}

pub fn checkutils(utilcmd: &str, cmdargs: &Vec<&str>) {
    match utilcmd {
        "cd" => {
            log_info("Ran builtin command: cd");
            run::builtin_cd(&cmdargs);
        }
        "ver" => {
            log_info("Ran builtin command: ver");
            run::bshversion();
        }
        "help" => {
            log_info("Ran builtin command: help");
            run::help();
        }
        "about" => {
            log_info("Ran builtin command: about");
            run::aboutbsh();
        }
        "exit" => {
            log_info("Exiting cell :(");
            run::exit();
        }
        "mkdir" => {
            log_info("Ran builtin command: mkdir");
            run::mkdir(&cmdargs);
        }
        "rm" => {
            log_info("Ran builtin command: rm");
            run::rm(&cmdargs);
        }
        "rmdir" => {
            log_info("Ran builtin command: rmdir");
            run::rmdir(&cmdargs);
        }
        "touch" => {
            log_info("Ran builtin command: touch");
            run::touch(&cmdargs);
        }
        "cp" => {
            log_info("Ran builtin command: cp");
            run::cp(&cmdargs);
        }
        "ls" => {
            log_info("Ran builtin command: ls");
            run::ls(&cmdargs);
        }
        "mv" => {
            log_info("Ran builtin command: mv");
            run::mv(&cmdargs);
        }
        _ => {
            log_error("Unknown command");
            println!(
                "{}",
                "Not available in bsh library, maybe it is not installed?".yellow()
            );
        }
    };
}

fn checkforbuiltin(cmd: &str) -> bool {
    matches!(cmd, "mkdir" | "rm" | "touch" | "mv" | "cp" | "rmdir" | "ls")
}
