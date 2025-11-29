#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashSet, env, ffi::OsStr, fs, os::unix::fs::PermissionsExt};

fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim_end().to_string();

        // Handle commands
        let set = HashSet::from(["exit", "type", "echo"]);
        if command == "exit" {
            return;
        } else if let Some(echo) = command.strip_prefix("echo ") {
            println!("{}", echo);
        } else if let Some(type_cmd) = command.strip_prefix("type ") {
            if set.contains(type_cmd) {
                println!("{} is a shell builtin", type_cmd);
                continue;
            }

            // Search through every dir in PATH if an executable file with the name type_cmd exists
            let mut found = false;
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

                    if entry.file_name() == OsStr::new(type_cmd) {
                        let file_path = entry.path();
                        println!("{} is {}", type_cmd, file_path.to_str().unwrap());
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                println!("{}: not found", type_cmd);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
