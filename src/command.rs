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
    io::{self, stdout, Write},
    path::PathBuf,
    process::{Command, Stdio},
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
    pub stdout_redirect: Option<String>,
    pub stderr_redirect: Option<String>,
}

impl ShellCommand {
    pub fn new(
        plain_command: String,
        args: Vec<String>,
        stdout_redirect: Option<String>,
        stderr_redirect: Option<String>,
    ) -> ShellCommand {
        ShellCommand {
            plain_command,
            args,
            stdout_redirect,
            stderr_redirect,
        }
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        let mut command = std::process::Command::new(&self.plain_command);
        command.args(&self.args);

        if let Some(path) = &self.stdout_redirect {
            let file = File::create(path)?;
            command.stdout(std::process::Stdio::from(file));
        }

        if let Some(path) = &self.stderr_redirect {
            // Ensure parent directory exists
            if let Some(parent) = std::path::Path::new(path).parent() {
                std::fs::create_dir_all(parent)?;
            }
            let file = File::create(path)?;
            command.stderr(std::process::Stdio::from(file));
        }

        let status = command
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    ShellError::CommandNotFound(self.plain_command.clone())
                } else {
                    ShellError::IoError(e)
                }
            })?
            .wait()?;

        if !status.success() {
            if let Some(code) = status.code() {
                return Err(ShellError::ExternalCommandFailed(code));
            }
        }

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
