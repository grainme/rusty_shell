//! Unix Shell Implementation
//!
//! @author: @grainme
//!
//! A module implementing built-in commands for a Unix-like shell.
//! This implementation provides basic shell functionality with
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

use crate::{environment::find_in_path, error::ShellError};
use std::{env, path::PathBuf};

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
            _ => None,
        }
    }
}

/// Core shell implementation handling built-in commands.
///
/// Maintains shell state and provides command execution functionality.
/// Currently focused on directory operations and command type checking.
pub struct Shell {
    /// Current working directory path (cached to reduce system calls)
    current_dir: PathBuf,
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
    pub fn pwd(&self) -> Result<&PathBuf, ShellError> {
        Ok(&self.current_dir)
    }

    /// Changes current directory (cd command).
    ///
    /// # Arguments
    /// * `path` - Target directory path
    ///
    /// # Errors
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
        let path = PathBuf::from(path);
        if path.is_dir() {
            env::set_current_dir(&path)?;
            self.current_dir = path;
        } else {
            return Err(ShellError::DirectoryNotFound);
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
    pub fn get_type(&self, option: &str) -> Result<String, ShellError> {
        if ShellCommandTypes::from_str(option).is_some() {
            Ok(format!("{} is a shell builtin", option))
        } else {
            match find_in_path(&option) {
                Some(res) => Ok(format!("{} is {}", option, res)),
                None => Err(ShellError::CommandNotFound(option.to_string())),
            }
        }
    }

    /// docs needed - todo
    ///
    ///
    pub fn exit(&self, code: i32) -> ! {
        std::process::exit(code);
    }

    /// docs needed - todo
    pub fn echo(&self, arg: String) -> String {
        arg
    }
}
