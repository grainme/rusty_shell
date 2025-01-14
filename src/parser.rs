//! Command line parsing
//!
//! Handles parsing of command line input:
//! - Command splitting
//! - Argument parsing
//! - Quote handling
//! - Token generation
//!
//! @author: @grainme
//!

use crate::{command::ShellCommand, error::ShellError};

/// To make it clear that the argument of parse_command
/// is a raw command instead of just `String`
type RawCommand = String;

/// Convert raw command into structured command
///
/// Design Questions:
/// How to handle malformed input?
/// Where to handle quoting/escaping?
/// Should it validate commands exist?
/// Split on spaces or more complex parsing?
///
pub fn parse_command(input: RawCommand) -> Result<ShellCommand, ShellError> {
    if input.trim().is_empty() {
        return Err(ShellError::CommandParsingFailed);
    }

    let (cmd, args): (&str, &str) = match input.as_str().split_once(" ") {
        Some(val) => val,
        None => (input.as_str(), ""),
    };

    Ok(ShellCommand::new(
        cmd.to_string(),
        args.split_whitespace().map(String::from).collect(),
    ))
}
