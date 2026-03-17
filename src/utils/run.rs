//use nothing for here
pub fn aboutbsh() {
    println!("Bucheshell is a shell developed for termux use cases");
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
    println!("Buche shell version 0.05 Alpha release");
}
pub fn help() {
    println!("built in commands -> cd, about, ver");
}
pub fn exit() {
    std::process::exit(0);
}
