#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    // Uncomment this block to pass the first stage
    let cmds = ["echo", "exit", "type"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let args: Vec<&str> = input.trim().split(" ").collect();
        match args[..] {
            ["exit", code] => process::exit(code.parse::<i32>().unwrap()),

            ["echo", ..] => println!("{}", args[1..].join(" ")),

            ["type", cmd] => {
                if cmds.contains(&cmd) {
                    println!("{} is a shell builtin", cmd)
                } else {
                    println!("{}: not found", cmd)
                }
            }
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
