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

#[derive(Copy, Clone)]
enum ParseState {
    InsideDoubleQuote,
    InsideSingleQuote,
    Outside,
    AfterEscapeChar,
}

fn parse_input(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    let mut state = ParseState::Outside;
    let mut prev_esc_char_state = state;
    let mut token = String::new();
    for c in input.chars() {
        match c {
            '\'' => match state {
                ParseState::InsideSingleQuote => state = ParseState::Outside,
                ParseState::Outside => state = ParseState::InsideSingleQuote,
                ParseState::AfterEscapeChar => {
                    state = prev_esc_char_state;
                    token.push(c);
                }
                _ => token.push(c),
            },
            '\\' => match state {
                ParseState::AfterEscapeChar => {
                    token.push(c);
                    state = prev_esc_char_state;
                }
                _ => {
                    prev_esc_char_state = state;
                    state = ParseState::AfterEscapeChar;
                    token.push(c);
                }
            },
            '\"' => match state {
                ParseState::AfterEscapeChar => {
                    state = prev_esc_char_state;
                    token.push(c);
                }
                ParseState::InsideDoubleQuote => state = ParseState::Outside,
                ParseState::Outside => state = ParseState::InsideDoubleQuote,
                _ => token.push(c),
            },
            ' ' => match state {
                ParseState::AfterEscapeChar => {
                    state = prev_esc_char_state;
                    token.push(c);
                }
                ParseState::InsideSingleQuote => token.push(c),
                ParseState::InsideDoubleQuote => token.push(c),
                _ => {
                    state = ParseState::Outside;
                    if !token.is_empty() {
                        tokens.push(token);
                        token = String::new();
                    }
                }
            },
            _ => token.push(c),
        }
    }

    if !token.is_empty() {
        tokens.push(token);
    }

    return tokens;
}

fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();
        let tokens = parse_input(&input);
        let cmd = tokens[0].to_string();

        // Handle commands
        let set = HashSet::from(["exit", "type", "echo", "pwd", "cd"]);
        if cmd == "exit" {
            return;
        } else if cmd == "pwd" {
            let cwd = env::current_dir().unwrap();
            println!("{}", cwd.display());
        } else if cmd == "echo" {
            println!("{}", tokens[1..].join(" "));
        } else if cmd == "cd" {
            let home = env::var("HOME").unwrap_or_default();
            let dir = tokens[1].replace("~", &home);
            match env::set_current_dir(&dir) {
                Ok(_) => {}
                Err(_) => println!("cd: {}: No such file or directory", dir),
            }
        } else if cmd == "type" {
            if set.contains(tokens[1].as_str()) {
                println!("{} is a shell builtin", tokens[1]);
                continue;
            }

            match find_exec(&tokens[1]) {
                Some(path) => println!("{} is {}", tokens[1], path.display()),
                None => println!("{}: not found", tokens[1]),
            }
        } else {
            if tokens.len() == 1 {
                println!("{}: command not found", cmd);
            }

            match find_exec(&cmd) {
                Some(_) => {
                    let output = Command::new(cmd)
                        .args(&tokens[1..])
                        .output()
                        .expect("Failed to run");
                    if output.stderr.is_empty() {
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        print!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                None => println!("{}: command not found", input),
            }
        }
    }
}
