//! Command Execution & Routing
//!
//! @author: @grainme
//!
//! A module implementing todo!("add stuff here")
//!
//! # Features
//!
//! # Usage Examples
//!

use crate::{builtins::Shell, error::ShellError, parser::parse_command};
use std::{
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub enum CommandOutput {
    Success,
    Text(String),
    Path(PathBuf),
}

#[derive(Debug)]
pub struct ShellCommand {
    pub plain_command: String,
    pub args: Vec<String>,
    pub stdout_redirect: Option<String>, // Store the output file path
}

impl ShellCommand {
    #[allow(dead_code)]
    pub fn new(
        plain_command: String,
        args: Vec<String>,
        stdout_redirect: Option<String>,
    ) -> ShellCommand {
        ShellCommand {
            plain_command,
            args,
            stdout_redirect,
        }
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        let mut command = std::process::Command::new(&self.plain_command);
        command.args(&self.args);

        if let Some(path) = &self.stdout_redirect {
            let file = File::create(path)?;

            command.stdout(std::process::Stdio::from(file));
            command.stderr(std::process::Stdio::inherit());
        } else {
            command.stdout(std::process::Stdio::inherit());
            command.stderr(std::process::Stdio::inherit());
        }

        let mut child = command.spawn()?;
        child.wait()?;

        Ok(())
    }
}

/// Reading raw input
///
/// we don't have any validation nor operations on
/// this function. we only return the user's input
/// whether it's a valid command or not.
///
/// Issues:
///  - Should read_command be implemented within Shell?
///  - Add empty line as ShellError variant ?
///
/// # Example
/// ```bash
/// > cd /directory
/// > pwd
/// > /directory
///
/// ```
///
fn read_input() -> Result<String, ShellError> {
    let mut command_args = String::new();
    std::io::stdin().read_line(&mut command_args)?;
    let command_args = command_args.trim().to_string();
    Ok(command_args)
}

pub fn run() -> Result<(), ShellError> {
    let mut shell = Shell::new()?;

    loop {
        print!("$ ");
        stdout().flush().map_err(ShellError::IoError)?;

        let raw_command = read_input()?;
        let command = parse_command(raw_command)?;

        match shell.execute_command(command) {
            Ok(output) => match output {
                CommandOutput::Success => (),
                CommandOutput::Text(text) => println!("{}", text),
                CommandOutput::Path(path) => println!("{}", path.display()),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
