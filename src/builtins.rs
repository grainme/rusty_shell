//! Built-in shell commands implementation
//!
//! contains implementations of shell built-in commands like:
//! current impls : cd, pwd
//!
//! @author: @grainme

use crate::{environment::find_in_path, error::ShellError};
use std::{env, path::PathBuf};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[non_exhaustive]
pub enum ShellCommand {
    Pwd,
    Cd,
    Ls,
    Echo,
    Cat,
    Type,
}

impl ShellCommand {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            ShellCommand::Pwd => "pwd",
            ShellCommand::Cd => "cd",
            ShellCommand::Cat => "cat",
            ShellCommand::Ls => "ls",
            ShellCommand::Type => "type",
            ShellCommand::Echo => "echo",
        }
    }

    pub fn from_str(command: &str) -> Option<ShellCommand> {
        match command {
            "pwd" => Some(ShellCommand::Pwd),
            "cd" => Some(ShellCommand::Cd),
            "cat" => Some(ShellCommand::Cat),
            "ls" => Some(ShellCommand::Ls),
            "type" => Some(ShellCommand::Type),
            "echo" => Some(ShellCommand::Echo),
            _ => None,
        }
    }
}

pub struct Shell {
    /// current_dir is used to cache the working
    /// directory instead of having multiple OS calls.
    current_dir: PathBuf,
}

impl Shell {
    pub fn new() -> Result<Shell, ShellError> {
        Ok(Shell {
            current_dir: env::current_dir()?,
        })
    }

    pub fn pwd(&self) -> Result<&PathBuf, ShellError> {
        Ok(&self.current_dir)
    }

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

    pub fn get_type(&self, option: &str) -> Result<String, ShellError> {
        // NotFound feels like an error, maybe todo!("re-implementation needed")
        if ShellCommand::from_str(option).is_some() {
            Ok(format!("{} is a shell builtin", option))
        } else {
            match find_in_path(&option) {
                Some(res) => Ok(format!("{} is {}", option, res)),
                None => Err(ShellError::FileNotFound),
            }
        }
    }
}
