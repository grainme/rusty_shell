//! Shell error types and handling
//!
//! @author: @grainme

use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
#[non_exhaustive]
pub enum ShellError {
    /// Returned when trying to access a directory that doesn't exist
    DirectoryNotFound,
    /// Returned when a file operation fails because the user lacks required permissions
    PermissionDenied,
    /// Returned when trying to access a file that doesn't exist
    FileNotFound,
    /// Returned when the path used is not a valid one
    InvalidPath,
    /// Returned when a command execution fails because the command was not found in PATH
    CommandNotFound,
    /// Returned when an I/O operation fails
    IoError(std::io::Error),
}

impl From<std::io::Error> for ShellError {
    fn from(error: std::io::Error) -> Self {
        ShellError::IoError(error)
    }
}

impl Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::DirectoryNotFound => write!(f, "Directory not found"),
            ShellError::PermissionDenied => write!(f, "Permission denied"),
            ShellError::FileNotFound => write!(f, "File not found"),
            ShellError::InvalidPath => write!(f, "Invalid path"),
            ShellError::CommandNotFound => write!(f, "Command not found"),
            ShellError::IoError(e) => write!(f, "I/O Error: {}", e),
        }
    }
}
