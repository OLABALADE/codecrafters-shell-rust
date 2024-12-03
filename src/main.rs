#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, process};

fn main() {
    let builtins = ["echo", "exit", "type"];
    let path = env::var("PATH").unwrap();
    let splits: Vec<&str> = path.as_str().split(":").collect();

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
                if builtins.contains(&cmd) {
                    println!("{} is a shell builtin", cmd);
                } else {
                    if let Some(path) = splits
                        .iter()
                        .find(|path| fs::metadata(format!("{}/{}", path, cmd)).is_ok())
                    {
                        println!("{cmd} is {path}/{cmd}")
                    } else {
                        println!("{cmd}: not found")
                    }
                }
            }
            _ => {
                if let Some(_) = splits
                    .iter()
                    .find(|path| fs::metadata(format!("{}/{}", path, args[0])).is_ok())
                {
                    let output = process::Command::new(args[0])
                        .args(&args[1..])
                        .output()
                        .expect("Failed to process command");

                    println!("{}", String::from_utf8_lossy(&output.stdout).trim())
                } else {
                    println!("{}: command not found", input.trim());
                }
            }
        }
    }
}
