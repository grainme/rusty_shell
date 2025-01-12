//! Command execution and routing
//!
//! tasks:
//!     - I need to make this code better. (120125)
//!
//! @author: @grainme

use crate::{
    builtins::{cd, pwd},
    environment::find_in_path,
    error::yell,
};
use std::{io::*, process::Command};

fn handle_pwd() {
    match pwd() {
        Ok(_) => {}
        Err(_) => println!("pwd panics"),
    };
}

fn handle_exit(args: &Vec<&str>) {
    let code = args.join(" ").trim().parse().unwrap_or(0);
    std::process::exit(code);
}

fn handle_type(args: &Vec<&str>) {
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

fn handle_cd(args: &Vec<&str>) {
    match cd(&args.join("")) {
        Ok(_) => {}
        Err(e) => println!("cd: {}: No such file or directory", e),
    }
}

pub fn run() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let stdin = stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input: Vec<_> = input.split_whitespace().map(|x| x.trim()).collect();

        if input.is_empty() {
            continue;
        }
        let cmd = input[0];
        let args: Vec<&str> = input[1..].to_vec();

        match cmd {
            "pwd" => handle_pwd(),
            "exit" => handle_exit(&args),
            "echo" => println!("{}", args.join(" ")),
            "cd" => handle_cd(&args),
            "type" => handle_type(&args),
            _ => match find_in_path(cmd) {
                Some(path) => {
                    if Command::new(path).args(&args).status().is_err() {
                        println!("{}: command not found", cmd);
                    }
                }
                None => println!("{}: command not found", cmd),
            },
        }
    }
}
