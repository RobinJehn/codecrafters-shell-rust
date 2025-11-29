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
        if command == "exit" {
            return;
        } else if let Some(echo) = command.strip_prefix("echo ") {
            println!("{}", echo);
        } else {
            println!("{}: command not found", command);
        }
    }
}
