//! Command execution and routing
//!
//! tasks:
//!     - I need to make this code better. (120125)
//!
//! @author: @grainme

use crate::{builtins::Shell, environment::find_in_path};
use std::{io::*, process::Command};

fn handle_exit(args: &Vec<&str>) {
    let code = args.join(" ").trim().parse().unwrap_or(0);
    std::process::exit(code);
}

pub fn run() {
    let mut shell: Shell = Shell::new().unwrap();
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
            "pwd" => shell.pwd(),
            "exit" => handle_exit(&args),
            "echo" => println!("{}", args.join(" ")),
            "cd" => shell.cd(&args.join("")).unwrap(),
            "type" => shell.type_s(&args),
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
