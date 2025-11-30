#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    collections::HashSet,
    env,
    ffi::OsStr,
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

fn find_exec(cmd: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();
    for dir in env::split_paths(&path) {
        let dir_cont = match fs::read_dir(dir) {
            Ok(value) => value,
            Err(_) => continue,
        };

        for entry in dir_cont {
            let entry = match entry {
                Ok(value) => value,
                Err(_) => continue,
            };

            let meta = match entry.metadata() {
                Ok(value) => value,
                Err(_) => continue,
            };

            if !meta.is_file() {
                continue;
            }

            let permissions = meta.permissions();
            let mode = permissions.mode();
            let executable = mode & 0o111;
            if executable == 0 {
                continue;
            }

            if entry.file_name() == OsStr::new(cmd) {
                let file_path = entry.path();
                return Some(file_path);
            }
        }
    }
    return None;
}

fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim_end().to_string();

        // Handle commands
        let set = HashSet::from(["exit", "type", "echo", "pwd", "cd"]);
        if command == "exit" {
            return;
        } else if command == "pwd" {
            let cwd = env::current_dir().unwrap();
            println!("{}", cwd.display());
        } else if let Some(echo) = command.strip_prefix("echo ") {
            println!("{}", echo);
        } else if let Some(dir) = command.strip_prefix("cd ") {
            let home = env::var("HOME").unwrap_or_default();
            let dir = dir.replace("~", &home);
            match env::set_current_dir(&dir) {
                Ok(_) => {}
                Err(_) => println!("cd: {}: No such file or directory", dir),
            }
        } else if let Some(type_cmd) = command.strip_prefix("type ") {
            if set.contains(type_cmd) {
                println!("{} is a shell builtin", type_cmd);
                continue;
            }

            match find_exec(type_cmd) {
                Some(path) => println!("{} is {}", type_cmd, path.display()),
                None => println!("{}: not found", type_cmd),
            }
        } else {
            let args: Vec<&str> = command.split_whitespace().collect();

            if args.is_empty() {
                println!("{}: command not found", command);
            }

            match find_exec(args[0]) {
                Some(_) => {
                    let output = Command::new(args[0])
                        .args(&args[1..])
                        .output()
                        .expect("Failed to run");

                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                None => println!("{}: command not found", command),
            }
        }
    }
}
