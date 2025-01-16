//! Shell environment management
//!
//! Handles all environment-related operations:
//! - PATH variable management
//! - Working directory operations
//!
//! @author: @grainme

use std::{env, path::Path};

pub fn search_bin(cmd: &str) -> Option<String> {
    let paths = env::var("PATH").ok()?;

    for dir in paths.split(":") {
        let file_path = Path::new(dir).join(cmd);
        if file_path.is_file() {
            return file_path.to_str().map(String::from);
        }
    }
    None
}
