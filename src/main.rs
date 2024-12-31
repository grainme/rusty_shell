use std::{env, fs, option};
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
fn get_file(cmd_needed: &str, paths_links: &str) -> Option<String> {
    let dirs_paths:Vec<&str> = paths_links.split(":").collect();
    for &dir_path in dirs_paths.iter() {
        // you need to fetch the files from `dirs`
        let files = fs::read_dir(dir_path).unwrap();
        for file_path in files {
            let file_path = file_path.unwrap().path();
            // check if file path dadada return some or none!
            let cmd_name: &str = file_path.file_name().take().unwrap().to_str().unwrap();
            if cmd_needed == cmd_name {
                return Some(format!("{cmd_needed} is {}", file_path.display()));
            }
        }
    }
    None
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
            "exit" => {
                std::process::exit(option
                    .trim()
                    .parse()
                    .expect("failed to parse exit code"))}, // normally i should handle this!
            "echo" => {
                println!("{}", option);
                },
            "type" => {
                if option == "echo" || option == "exit" {
                    println!("{} is a shell builtin", option);
                } else {
                    match get_file(option, &env::var("PATH").unwrap()) {
                        Some(res) => println!("{}", res),
                        None => yell(option), 
                    }
                }
            }
            _ => println!("{}: command not found", cmd),
        }
    }
}
