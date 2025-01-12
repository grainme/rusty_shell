//! Built-in shell commands implementation
//!
//! contains implementations of shell built-in commands like:
//! current impls : cd, pwd
//!
//! @author: @grainme

use std::{env, error::Error, path::PathBuf};

struct Shell {
    current_dir: PathBuf,
}

impl Shell {
    pub fn new() -> Result<Shell, Box<dyn Error>> {
        Ok(Shell {
            current_dir: env::current_dir()?,
        })
    }

    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn pwd(&self) {
        println!("{}", self.get_current_dir().display());
    }

    pub fn cd(&mut self, _path: &str) -> Result<(), &str> {
        todo!()
    }
}
