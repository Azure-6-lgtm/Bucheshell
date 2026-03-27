use std::fs;
use std::fs::File;
pub fn aboutbsh() {
    println!("Bucheshell is a shell that might be cool ig? :3");
}
pub fn ls(args: &[&str]) {
    let mut owned: Option<String> = None;

    let mut path = args.get(0).copied().unwrap_or(".");

    if path.starts_with("~") {
        if let Ok(home) = std::env::var("HOME") {
            let replaced = path.replacen("~", &home, 1);
            owned = Some(replaced);
            path = owned.as_deref().unwrap();
        }
    }

    match std::fs::read_dir(path) {
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
    use std::env;
    use std::path::PathBuf;

    let home = env::var("HOME").unwrap_or(String::from("/"));

    let target = if args.is_empty() {
        PathBuf::from(&home)
    } else if args[0].starts_with("~") {
        PathBuf::from(args[0].replacen("~", &home, 1))
    } else {
        PathBuf::from(args[0])
    };

    if let Err(e) = env::set_current_dir(&target) {
        println!("cd: {}", e);
    }
}
pub fn bshversion() {
    println!("Buche shell version 1.0 release");
}
pub fn help() {
    println!("built in commands -> cd, about, ver,touch,mkdir,rmdir,rm,ls");
}
pub fn exit() {
    std::process::exit(0);
}
pub fn mkdir(args: &[&str]) {
    if let Some(dir) = args.get(0) {
        if let Err(e) = fs::create_dir(dir) {
            eprintln!("mkdir: {}", e);
        }
    } else {
        eprintln!("mkdir: missing operand");
    }
}

pub fn touch(args: &[&str]) {
    if let Some(file) = args.get(0) {
        if let Err(e) = File::create(file) {
            eprintln!("touch: {}", e);
        }
    } else {
        eprintln!("touch: missing file");
    }
}

pub fn rm(args: &[&str]) {
    if let Some(file) = args.get(0) {
        if let Err(e) = fs::remove_file(file) {
            eprintln!("rm: {}", e);
        }
    } else {
        eprintln!("rm: missing file");
    }
}

pub fn rmdir(args: &[&str]) {
    if let Some(dir) = args.get(0) {
        if let Err(e) = fs::remove_dir(dir) {
            eprintln!("rmdir: {}", e);
        }
    } else {
        eprintln!("rmdir: missing dir");
    }
}

/*pub fn ls(args: &[&str]) {
    let path = args.get(0).map(|s| ).unwrap_or(".");

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                print!("{}  ", entry.file_name().to_string_lossy());
            }
            println!();
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}*/

pub fn cp(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("cp: src dst");
        return;
    }

    if let Err(e) = fs::copy(&args[0], &args[1]) {
        eprintln!("cp: {}", e);
    }
}

pub fn mv(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("mv: src dst");
        return;
    }

    if let Err(e) = fs::rename(&args[0], &args[1]) {
        eprintln!("mv: {}", e);
    }
}
