#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, process};

fn main() {
    // Uncomment this block to pass the first stage
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
                    search_cmd(&splits, cmd);
                }
            }
            _ => println!("{}: command not found", input.trim()),
        }
    }
}

fn search_cmd(paths: &Vec<&str>, cmd: &str) {
    let mut found = false;
    'outer: for p in paths {
        if let Ok(entries) = fs::read_dir(p) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.file_name() == cmd {
                        found = true;
                        println!("{} is {}", cmd, entry.path().to_str().unwrap());
                        break 'outer;
                    }
                }
            }
        }
    }
    if !found {
        println!("{}: not found", cmd)
    }
}
