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
        return Err(ShellError::EmptyCommand);
    }

    let mut chars = input.chars().peekable();
    let mut command = String::new();
    let mut args = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;

    // parsing command
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() && !command.is_empty() {
            chars.next();
            break;
        }
        command.push(chars.next().unwrap());
    }

    while let Some(c) = chars.next() {
        match c {
            '\'' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !current_token.is_empty() {
                    args.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => current_token.push(c),
        }
    }

    if !current_token.is_empty() {
        args.push(current_token);
    }

    // in case opened but not closed
    if in_quotes {
        return Err(ShellError::UnmatchedQuote);
    }

    // parsing arguments
    Ok(ShellCommand::new(command, args))
}
