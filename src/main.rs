use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{self, Write};

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
            } else {
                println!("{}: not found", type_cmd);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}
