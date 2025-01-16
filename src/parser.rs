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

    let mut is_inside = false;
    let mut is_space = false;
    let mut word = String::new();
    let mut arguments: Vec<String> = Vec::new();

    for c in args.chars().collect::<Vec<char>>() {
        if c == '\'' {
            is_inside = !is_inside;
            continue;
        }
        if is_inside {
            word.push(c);
        } else {
            if c.is_whitespace() && is_space {
                continue;
            } else if c.is_whitespace() {
                arguments.push(word.trim().to_string());
                word.clear();
                is_space = true;
            } else {
                is_space = false;
            }
            word.push(c);
        }
    }
    if !word.is_empty() {
        arguments.push(word.trim().to_string());
    }
    Ok(ShellCommand::new(cmd.to_string(), arguments))
}
