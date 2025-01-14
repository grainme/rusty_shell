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
    /// Returned when the operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// Returned when trying to access a file that doesn't exist
    FileNotFound,
    /// Returned when the path used is not a valid one
    InvalidPath,
    /// Returned when a command execution fails because the command was not found in PATH
    CommandNotFound(String),
    /// Returned when a command parsing fails
    CommandParsingFailed,
    /// Returned when an I/O operation fails
    IoError(std::io::Error),
}

impl From<std::io::Error> for ShellError {
    fn from(error: std::io::Error) -> Self {
        ShellError::IoError(error)
    }
}

impl ShellError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ShellError::DirectoryNotFound => "directory not found",
            ShellError::PermissionDenied => "permission denied",
            ShellError::FileNotFound => "file not found",
            ShellError::InvalidPath => "invalid path",
            ShellError::CommandNotFound(_) => "not found",
            Self::CommandParsingFailed => "command parsing failed",
            ShellError::IoError(_) => "i/o error",
        }
    }
}

impl Display for ShellError {
    /// Shows a human-readable description of the `ShellError`.
    ///
    /// This is similar to `impl Display for ErrorKind`.
    ///
    /// # Examples
    /// ```
    /// use std::io::ErrorKind;
    /// assert_eq!("permission denied", ErrorKind::PermissionDenied.to_string());
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{}", e),
            Self::CommandNotFound(cmd) => write!(f, "{}: {}", cmd, self.as_str()),
            _ => f.write_str(self.as_str()),
        }
    }
}
