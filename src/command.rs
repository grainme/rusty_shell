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

use crate::{
    builtins::{Shell, ShellCommandTypes},
    error::ShellError,
    parser::parse_command,
};
use std::{
    io::{stdout, Write},
    path::PathBuf,
};

#[allow(dead_code)]
enum CommandOutput {
    // for commands that succeed but produce no output like (cd)
    Success,
    // for commands that produce output like (type)
    Text(String),
    // for commands that produce path output like (pwd)
    Path(PathBuf),
    // for when commands fail
    Error(ShellError),
}

#[derive(Debug)]
pub struct ShellCommand {
    plain_command: String,
    args: Vec<String>,
}

impl ShellCommand {
    pub fn new(plain_command: String, args: Vec<String>) -> ShellCommand {
        ShellCommand {
            plain_command,
            args,
        }
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

///
/// Execute command
///
fn execute_command(shell: &mut Shell, command: ShellCommand) -> Result<CommandOutput, ShellError> {
    let shell_command_type: ShellCommandTypes =
        match ShellCommandTypes::from_str(&command.plain_command) {
            Some(cmd_type) => cmd_type,
            None => return Err(ShellError::CommandNotFound(command.plain_command)),
        };

    match shell_command_type {
        ShellCommandTypes::Cd => {
            shell.cd(&command.args.concat())?;
            return Ok(CommandOutput::Success);
        }
        ShellCommandTypes::Pwd => {
            let path = shell.pwd()?;
            return Ok(CommandOutput::Path(path.clone()));
        }
        ShellCommandTypes::Type => {
            let cmd = shell.get_type(&command.args.concat())?;
            return Ok(CommandOutput::Text(cmd));
        }
        ShellCommandTypes::Echo => {
            let arg = command.args.join(" ");
            return Ok(CommandOutput::Text(shell.echo(arg)));
        }
        ShellCommandTypes::Exit => {
            let code = match command.args.concat().parse::<i32>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(ShellError::CommandParsingFailed);
                }
            };
            shell.exit(code);
        }
    }
}

pub fn run() {
    let mut shell: Shell = match Shell::new() {
        Ok(shell) => shell,
        Err(e) => {
            eprintln!("failed to init shell: {}", e);
            return;
        }
    };

    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let raw_command = match read_input() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error reading input: {}", e);
                return;
            }
        };

        let shell_command = match parse_command(raw_command) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error parsing input: {}", e);
                return;
            }
        };

        match execute_command(&mut shell, shell_command) {
            Ok(cmd_output) => {
                match cmd_output {
                    CommandOutput::Success => {}
                    CommandOutput::Text(text) => {
                        println!("{}", text);
                    }
                    CommandOutput::Path(path) => {
                        println!("{}", path.display());
                    }
                    CommandOutput::Error(e) => {
                        // not sure about this error yet!
                        eprintln!("error {}", e);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
