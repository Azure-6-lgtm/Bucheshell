use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

/* =========================
   Helper: expand ~
   ========================= */
fn expand_tilde(input: &str) -> PathBuf {
    if input == "~" || input.starts_with("~/") {
        if let Some(home) = env::var_os("HOME") {
            let mut path = PathBuf::from(home);

            if input.len() > 2 {
                path.push(&input[2..]); // skip "~/"
            }

            return path;
        }
    }

    PathBuf::from(input)
}

/* =========================
   Basic commands
   ========================= */

pub fn aboutbsh() {
    println!("Bucheshell is a shell that might be cool ig? :3");
}

pub fn bshversion() {
    println!("Buche shell version 2.2.0 release");
}

pub fn help() {
    println!("Built in commands -> cd, about, ver, touch, mkdir, rmdir, rm, ls, export.");
    println!("You can set custom aliases in \"~/.bucherc\"");
}

pub fn exit() {
    std::process::exit(0);
}

/* =========================
   File system commands
   ========================= */

pub fn ls(args: &[&str]) {
    let path = args.get(0).copied().unwrap_or(".");
    let path = expand_tilde(path);

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                print!("{}  ", entry.file_name().to_string_lossy());
            }
            println!();
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}

pub fn builtin_cd(args: &[&str]) {
    let target = if args.is_empty() {
        expand_tilde("~")
    } else {
        expand_tilde(args[0])
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}", e);
    }
}

pub fn mkdir(args: &[&str]) {
    if let Some(dir) = args.get(0) {
        let path = expand_tilde(dir);

        if let Err(e) = fs::create_dir_all(path) {
            eprintln!("mkdir: {}", e);
        }
    } else {
        eprintln!("mkdir: missing operand");
    }
}

pub fn touch(args: &[&str]) {
    if let Some(file) = args.get(0) {
        let path = expand_tilde(file);

        if let Err(e) = File::create(path) {
            eprintln!("touch: {}", e);
        }
    } else {
        eprintln!("touch: missing file");
    }
}

pub fn rm(args: &[&str]) {
    if let Some(file) = args.get(0) {
        let path = expand_tilde(file);

        if let Err(e) = fs::remove_file(path) {
            eprintln!("rm: {}", e);
        }
    } else {
        eprintln!("rm: missing file");
    }
}

pub fn rmdir(args: &[&str]) {
    if let Some(dir) = args.get(0) {
        let path = expand_tilde(dir);

        if let Err(e) = fs::remove_dir(path) {
            eprintln!("rmdir: {}", e);
        }
    } else {
        eprintln!("rmdir: missing dir");
    }
}

pub fn cp(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("cp: src dst");
        return;
    }

    let src = expand_tilde(args[0]);
    let dst = expand_tilde(args[1]);

    if let Err(e) = fs::copy(src, dst) {
        eprintln!("cp: {}", e);
    }
}

pub fn mv(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("mv: src dst");
        return;
    }

    let src = expand_tilde(args[0]);
    let dst = expand_tilde(args[1]);

    if let Err(e) = fs::rename(src, dst) {
        eprintln!("mv: {}", e);
    }
}
