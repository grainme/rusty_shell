#[allow(unused_imports)]
use std::io::{self, Write};


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

        match cmd {
            "exit" => {
            std::process::exit(input.get(1)
                .unwrap()
                .parse()
                .expect("failed to parse exit code"))
             },
            "echo" => {
                println!("{}", &input[1..].join(" "));
            },
            _ => println!("{}: command not found", cmd),
        }
    }
}
