#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let args: Vec<&str> = input.trim().split(" ").collect();
        match args[0] {
            "exit" => {
                let code: i32 = args[1].parse().expect("Invalid code");
                process::exit(code)
            }
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
