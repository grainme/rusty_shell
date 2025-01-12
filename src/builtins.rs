//! Built-in shell commands implementation
//!
//! contains implementations of shell built-in commands like:
//! current impls : cd, pwd
//!
//! @author: @grainme

use crate::{environment::find_in_path, error::ShellError};
use std::{collections::HashSet, env, path::PathBuf};

pub struct Shell {
    /// current_dir is used to cache the working
    /// directory instead of having multiple OS calls.
    current_dir: PathBuf,
    commands: HashSet<&'static str>,
}

impl Shell {
    pub fn new() -> Result<Shell, ShellError> {
        let commands_set: HashSet<&'static str> =
            HashSet::from_iter(vec!["pwd", "cd", "ls", "echo"]);
        Ok(Shell {
            current_dir: env::current_dir()?,
            commands: commands_set,
        })
    }

    ///
    /// we're calling env::current_dir once within new
    /// in get_current_dir we're just fetching it from the
    /// Shell instance.
    ///
    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn pwd(&self) {
        println!("{}", self.get_current_dir().display());
    }

    ///
    /// case where cd would fail?
    ///     - not found path ? - "cd: No such file or directory : path"
    ///
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

    ///
    /// type is used to find out whether command is builtin
    /// or external binary.
    ///
    /// Usage:
    /// ```
    /// type echo
    /// echo is a shell builtin
    /// ```
    ///
    pub fn type_s(&self, args: &Vec<&str>) {
        let binding = args.join(" ");
        let option: &str = binding.as_str();

        if self.commands.contains(&option) {
            println!("{} is a shell builtin", option);
        } else {
            match find_in_path(&option) {
                Some(res) => println!("{} is {}", option, res),
                None => println!("{}: not found", option),
            }
        }
    }
}
