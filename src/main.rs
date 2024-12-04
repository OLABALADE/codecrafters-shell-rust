#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, path::Path, process};

fn main() {
    let builtins = ["echo", "exit", "type", "pwd", "cd"];
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

            ["type", cmd] => ty(&builtins, &splits, cmd),

            ["pwd", ..] => pwd(),

            ["cd", pat] => cd(pat),

            _ => check_path(&splits, &args),
        }
    }
}

fn ty(builtins: &[&str], splits: &Vec<&str>, cmd: &str) {
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

fn check_path(splits: &Vec<&str>, args: &Vec<&str>) {
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
        println!("{}: command not found", args[0]);
    }
}

fn pwd() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display())
}

fn cd(path: &str) {
    let new_path = Path::new(path);
    if !env::set_current_dir(new_path).is_ok() {
        println!("cd: {path}: No such file or directory")
    }
}
