#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::{self, Path}, process::Command};

use anyhow::Result;

#[allow(dead_code)]
fn get_type_response(arg: &str) {
    if !["echo", "type", "exit"].contains(&arg) {
        println!("{arg}: not found");
    } else {
        println!("{arg} is a shell builtin");
    }
}

fn yell(arg: &str) {
    println!("{}: not found", arg);
}

#[allow(unused_variables)]
#[allow(dead_code)]
// takes a file and look it up within the dirs!
fn find_in_path(cmd: &str) -> Option<String> {
    let paths = env::var("PATH").ok()?;

    for dir in paths.split(":") {
        let file_path = Path::new(dir).join(cmd);
        if file_path.is_file() {
            return file_path.to_str().map(String::from);
        }
    }
    None
}

fn pwd() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    println!("{}", current_dir.display());
    Ok(())
}

fn change_dir(path: &str) -> Result<(), &str> {
    let home;
    let root = match path {
        "~" => {
            home = env::var("HOME").unwrap();
            Path::new(&home)
        },
        _ => Path::new(path),
    };
    let root = env::set_current_dir(&root).is_ok();
    if !root {
        return Err(path);
    }
    Ok(())
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input: Vec<_> = input.split_whitespace().map(|x| x.trim()).collect();

        if input.is_empty() {
            continue;
        }
        let cmd = input[0];
        let args: Vec<&str> = input[1..].to_vec();

        match cmd {
            "pwd" => match pwd() {
                Ok(_) => {}
                Err(_) => println!("pwd panics"),
            },
            "exit" => {
                let code = args.join(" ").trim().parse().unwrap_or(0);
                std::process::exit(code);
            }
            "echo" => {
                println!("{}", args.join(" "));
            },
            "cd" => {
                match change_dir(&args.join("")) {
                    Ok(_) => {},
                    Err(e) => println!("cd: {}: No such file or directory", e),
                }
            }
            "type" => {
                let option = args.join(" ");
                if option == "echo" || option == "exit" || option == "type" || option == "pwd" {
                    println!("{} is a shell builtin", option);
                } else {
                    match find_in_path(&option) {
                        Some(res) => println!("{}", res),
                        None => yell(&option),
                    }
                }
            }
            _ => {
                match find_in_path(cmd) {
                    Some(path) => {
                        if Command::new(path)
                            .args(&args)
                            .status().is_err() {
                            println!("{}: command not found", cmd);
                        }
                    }
                    None => println!("{}: command not found", cmd),
                }
            }
        }
    }
}
