#[allow(unused_imports)]
use std::io::{self, Write};


fn get_type_response(arg: &str) {
    if !["echo", "type", "exit"].contains(&arg) {
        println!("{arg}: not found");
    } else {
        println!("{arg} is a shell builtin");
    }
}

fn main() {
    // Wait for user input
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
        let option = &input[1..].join(" ");

        match cmd {
            "type" => get_type_response(option),
            "exit" => {
                std::process::exit(input.get(1)
                    .unwrap()
                    .parse()
                    .expect("failed to parse exit code"))},
            "echo" => {
                println!("{}", option);
            },
            _ => println!("{}: command not found", cmd),
        }
    }
}
