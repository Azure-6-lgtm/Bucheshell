use crate::utils::run;
use colored::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use crate::logger::buchelog::{log_info, log_warn, log_error};

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
        //Command::new(&termuxpath);
        log_info("Ran command succesfully");
        Command::new(&termuxpath)
        //log_info("Running command : {termuxpath}")
    } else if Path::new(&userpath).exists() && !checkforbuiltin(cmd) {
        //Command::new(&userpath);
        log_info("Ran command succesfully");
        Command::new(&userpath)
    } else if Path::new(&shellpath).exists() && !checkforbuiltin(cmd) {
        //Command::new(&shellpath);
        log_info("Ran command succesfully");
        Command::new(&shellpath)
    } else {
        checkutils(&cmd, &cmdargs);
        log_warn("Passing unknown command to utils checker");
        return; 
    };

    command_to_run.args(&cmdargs);

    if let Some(file) = output_file {
        // Open the file for appending
        let file_handle = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file)
            .expect("Failed to open file for appending");
        command_to_run.stdout(Stdio::from(file_handle));
    }

    command_to_run
        .status()
        .expect("Bucheshell failed to run");
}

pub fn checkutils(utilcmd: &str, cmdargs: &Vec<&str>) {
    match utilcmd {
        "cd" => run::builtin_cd(&cmdargs),
        "ver" => run::bshversion(),
        "help" => run::help(),
        "about" => run::aboutbsh(),
        "exit" => run::exit(),
        "mkdir" => run::mkdir(&cmdargs),
        "rm" => run::rm(&cmdargs),
        "rmdir" => run::rmdir(&cmdargs),
        "touch" => run::touch(&cmdargs),
        "cp" => run::cp(&cmdargs),
        "ls" => run::ls(&cmdargs),
        "mv" => run::mv(&cmdargs),
        _ => {
            log_error("Unknown command");
            println!("{}", "Not available in bsh library, maybe it is not installed?".yellow());
        }
    };
}

fn checkforbuiltin(cmd: &str) -> bool {
    matches!(cmd, "mkdir" | "rm" | "touch" | "mv" | "cp" | "rmdir" | "ls")
}
