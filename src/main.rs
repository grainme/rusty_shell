use std::{env, path::Path, process::Command};
#[allow(unused_imports)]
use std::io::{self, Write};

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
            return file_path.to_str().map(String::from)
        }
    }
    None
}

fn pwd() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    println!("{}", current_dir.display());
    Ok(())
}


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input: Vec<_> = input
            .split_whitespace()
            .map(|x| x.trim())
            .collect();
        
        // dbg!(&input);
        let &cmd = input.first().unwrap();
        let option: &str = &input[1..].join(" ");
        match cmd {
            "pwd" =>  {
                match pwd() {
                    Ok(_) => {},
                    Err(_) => println!("pwd panics"),
                }
            },
            "exit" => {
                std::process::exit(option
                    .trim()
                    .parse()
                    .expect("failed to parse exit code"))}, // normally i should handle this!
            "echo" => {
                println!("{}", option);
                },
            "type" => {
                if option == "echo" || option == "exit" || option == "type" {
                    println!("{} is a shell builtin", option);
                } else {
                    match find_in_path(option) {
                        Some(res) => println!("{}", res),
                        None => yell(option), 
                    }
                }
            }
            _ => {
                match Command::new(cmd).args(option.split(" ")).status() {
                    Ok(_) => {} ,
                    Err(_) => {
                        println!("{cmd}: command not found");
                    },
                }
            },
        }
    }
}
