//! Unix Shell Implementation
//!
//! @author: @grainme
//!
//! A module implementing built-in commands for a Unix-like self.
//! This implementation provides(or should) basic shell functionality with
//! support for built-in commands and external program execution.
//!
//! # Features
//!
//! Built-in commands:
//! * `pwd` - Print working directory
//! * `cd` - Change directory
//! * `type` - Display command type
//!
//! # Usage Examples
//!
//! ```bash
//! $ my-shell
//! > pwd
//! /current/directory
//! > cd /tmp
//! > type pwd
//! pwd is a shell builtin
//! ```

use crate::{
    command::{CommandOutput, ShellCommand},
    environment::search_bin,
    error::ShellError,
};
use std::{env, io::ErrorKind, path::PathBuf, process::Command};

/// Built-in shell commands supported by this implementation.
///
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[non_exhaustive]
pub enum ShellCommandTypes {
    /// pwd: Print current working directory
    Pwd,
    /// cd: Change current directory
    Cd,
    /// ls: List directory contents (unimplemented)
    Echo,
    /// type: Show command type information
    Type,
    // exit: quits the shell (with a status code)
    Exit,
    // ls: Get all the files of a path (default: working directory)
    Ls,
    // TODO: add doc
    Cat,
    // TODO: add doc
    Clear,
}

impl ShellCommandTypes {
    /// Converts a shell command string into its enum representation.
    ///
    /// # Arguments
    /// * `command` - Command name (e.g., "pwd", "cd")
    ///
    /// # Returns
    /// * `Some(ShellCommandTypes)` for recognized commands
    /// * `None` for unknown commands
    pub fn from_str(command: &str) -> Option<ShellCommandTypes> {
        match command {
            "pwd" => Some(ShellCommandTypes::Pwd),
            "cd" => Some(ShellCommandTypes::Cd),
            "type" => Some(ShellCommandTypes::Type),
            "echo" => Some(ShellCommandTypes::Echo),
            "exit" => Some(ShellCommandTypes::Exit),
            "ls" => Some(ShellCommandTypes::Ls),
            "clear" => Some(ShellCommandTypes::Clear),
            _ => None,
        }
    }
}

/// Core shell implementation handling built-in commands.
///
/// Maintains shell state and provides command execution functionality.
/// Currently focused on directory operations and command type checking.
#[allow(dead_code)]
pub struct Shell {
    /// TODO: NO USE CASE SO FAR - Current working directory path (cached to reduce system calls)
    current_dir: PathBuf,
    // For tracking exit status of commands
    // last_status: i32,
}

impl Shell {
    /// Creates a new shell instance in the current directory.
    ///
    /// # Errors
    /// Returns ShellError if unable to determine current directory
    pub fn new() -> Result<Shell, ShellError> {
        Ok(Shell {
            current_dir: env::current_dir()?,
        })
    }

    pub fn execute_command(&mut self, command: ShellCommand) -> Result<CommandOutput, ShellError> {
        // Early return for empty commands
        if command.plain_command.is_empty() {
            return Err(ShellError::EmptyCommand);
        }

        // Try to execute as built-in command first
        match ShellCommandTypes::from_str(&command.plain_command) {
            Some(cmd_type) => self.execute_builtin(cmd_type, command),
            None => self.execute_external(command),
        }
    }

    /// Shows current working directory (pwd command).
    ///
    /// # Errors
    /// Returns ShellError if directory access fails
    ///
    /// # Example Output
    /// ```bash
    /// > pwd
    /// /home/user
    /// ```
    pub fn pwd(&self) -> Result<PathBuf, ShellError> {
        Ok(env::current_dir()?)
    }

    /// Changes current directory (cd command).
    ///
    /// # Arguments
    /// * `path` - Target directory path
    ///
    /// # Errors (some cases)
    /// * ShellError::DirectoryNotFound if directory doesn't exist
    /// * ShellError if permission denied or other access error
    ///
    /// # Example Usage
    /// ```bash
    /// > cd /tmp
    /// > pwd
    /// /tmp
    /// ```
    pub fn cd(&mut self, path: &str) -> Result<(), ShellError> {
        #[allow(deprecated)]
        let binding = match env::home_dir() {
            Some(path) => path,
            None => return Err(ShellError::HomeDirNotFound),
        };

        // TODO: handle unwrap here, should we?
        let path = match path {
            "~" => binding.to_str().unwrap(),
            _ => path,
        };

        let cd_result = env::set_current_dir(&path);
        if cd_result.is_err() {
            return Err(ShellError::FileAndDirectoryNotFound(
                "cd".to_string(),
                path.to_string(),
            ));
        }
        Ok(())
    }

    /// Shows command type information (type command).
    ///
    /// # Arguments
    /// * `option` - Name of command to check
    ///
    /// # Returns
    /// * String describing if command is built-in or external
    /// * ShellError::FileNotFound if command doesn't exist
    ///
    /// # Example Output
    /// ```bash
    /// > type pwd
    /// pwd is a shell builtin
    /// > type ls
    /// ls is /bin/ls
    /// ```
    pub fn type_(&self, command: ShellCommand) -> Result<CommandOutput, ShellError> {
        let option = command.args.concat();
        if ShellCommandTypes::from_str(&option).is_some() {
            Ok(CommandOutput::Text(format!(
                "{} is a shell builtin",
                &option
            )))
        } else {
            match search_bin(&option) {
                Some(res) => Ok(CommandOutput::Text(format!("{} is {}", option, res))),
                None => Err(ShellError::CommandNotFound(option.to_string())),
            }
        }
    }

    /// exit
    /// docs needed - todo
    ///
    pub fn exit(&self, code: i32) -> ! {
        std::process::exit(code);
    }

    /// echo
    /// docs needed - todo
    ///
    pub fn echo(&self, arg: String) -> Result<CommandOutput, ShellError> {
        Command::new("echo").arg(arg).status()?;
        Ok(CommandOutput::Success)
    }

    /// ls
    /// docs needed
    ///
    pub fn ls(&self) -> Result<CommandOutput, ShellError> {
        Command::new("ls").status()?;
        Ok(CommandOutput::Success)
    }

    pub fn clear(&self) -> Result<CommandOutput, ShellError> {
        Command::new("clear").status()?;
        Ok(CommandOutput::Success)
    }

    pub fn cat(&self, command: ShellCommand) -> Result<CommandOutput, ShellError> {
        Command::new("cat").args(command.args).status()?;
        Ok(CommandOutput::Success)
    }

    pub fn execute_external(&self, command: ShellCommand) -> Result<CommandOutput, ShellError> {
        Command::new(&command.plain_command)
            .args(&command.args)
            .status()
            .map_err(|e| match e.kind() {
                ErrorKind::NotFound => ShellError::CommandNotFound(command.plain_command),
                _ => ShellError::IoError(e),
            })?;
        Ok(CommandOutput::Success)
    }

    ///
    /// Execute command
    ///
    pub fn execute_builtin(
        &mut self,
        cmd_type: ShellCommandTypes,
        command: ShellCommand,
    ) -> Result<CommandOutput, ShellError> {
        match cmd_type {
            ShellCommandTypes::Cd => {
                self.cd(&command.args.concat())?;
                return Ok(CommandOutput::Success);
            }
            ShellCommandTypes::Pwd => {
                let path = self.pwd()?;
                return Ok(CommandOutput::Path(path.clone()));
            }
            ShellCommandTypes::Type => {
                return Ok(self.type_(command)?);
            }
            ShellCommandTypes::Echo => {
                let arg = command.args.join(" ");
                self.echo(arg)?;
                return Ok(CommandOutput::Success);
            }
            // Not complete!
            ShellCommandTypes::Ls => {
                // TODO: change this to something else later
                self.ls()?;
                return Ok(CommandOutput::Success);
            }
            ShellCommandTypes::Cat => {
                self.cat(command)?;
                return Ok(CommandOutput::Success);
            }
            ShellCommandTypes::Clear => {
                self.clear()?;
                return Ok(CommandOutput::Success);
            }
            ShellCommandTypes::Exit => {
                let code = command
                    .args
                    .concat()
                    .parse()
                    .map_err(|_| ShellError::CommandParsingFailed)?;
                self.exit(code);
            }
        }
    }
}
