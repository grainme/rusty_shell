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

/// Convert raw command into structured command
///
/// Design Questions:
/// How to handle malformed input?
/// Where to handle quoting/escaping?
/// Should it validate commands exist?
/// Split on spaces or more complex parsing?
///
use crate::{command::ShellCommand, error::ShellError};

type RawCommand = String;

#[derive(Debug)]
pub enum ParseError {
    UnmatchedSingleQuote,
    UnmatchedDoubleQuote,
    EmptyCommand,
}

impl From<ParseError> for ShellError {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::UnmatchedSingleQuote => {
                ShellError::UnmatchedQuote("single quote".to_string())
            }
            ParseError::UnmatchedDoubleQuote => {
                ShellError::UnmatchedQuote("double quote".to_string())
            }
            ParseError::EmptyCommand => ShellError::EmptyCommand,
        }
    }
}

pub fn parse_command(input: RawCommand) -> Result<ShellCommand, ShellError> {
    if input.trim().is_empty() {
        return Err(ParseError::EmptyCommand.into());
    }

    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut stdout_redirect = None;
    let mut stderr_redirect = None;

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if in_single_quotes {
                    current_token.push('\\');
                } else if in_double_quotes {
                    if let Some(&next_char) = chars.peek() {
                        match next_char {
                            '$' | '`' | '"' | '\\' | '\n' => {
                                chars.next();
                                current_token.push(next_char);
                            }
                            _ => {
                                current_token.push('\\');
                                current_token.push(next_char);
                                chars.next();
                            }
                        }
                    } else {
                        current_token.push('\\');
                    }
                } else {
                    if let Some(&next_char) = chars.peek() {
                        chars.next();
                        current_token.push(next_char);
                    } else {
                        current_token.push('\\');
                    }
                }
            }
            '\'' => {
                if in_double_quotes {
                    current_token.push('\'');
                } else {
                    in_single_quotes = !in_single_quotes;
                }
            }
            '"' => {
                if in_single_quotes {
                    current_token.push('"');
                } else {
                    in_double_quotes = !in_double_quotes;
                }
            }
            c if c.is_whitespace() => {
                if in_single_quotes || in_double_quotes {
                    current_token.push(c);
                } else if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => current_token.push(c),
        }
    }

    if in_single_quotes {
        return Err(ParseError::UnmatchedSingleQuote.into());
    }
    if in_double_quotes {
        return Err(ParseError::UnmatchedDoubleQuote.into());
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    if tokens.is_empty() {
        return Err(ParseError::EmptyCommand.into());
    }

    while let Some(token) = tokens.iter().position(|t| t == ">" || t == "1>") {
        if token + 1 < tokens.len() {
            stdout_redirect = Some(tokens[token + 1].clone());
            tokens.drain(token..=token + 1);
        } else {
            return Err(ShellError::EmptyCommand);
        }
    }

    while let Some(token) = tokens.iter().position(|t| t == "2>") {
        if token + 1 < tokens.len() {
            stderr_redirect = Some(tokens[token + 1].clone());
            tokens.drain(token..=token + 1);
        } else {
            return Err(ShellError::EmptyCommand);
        }
    }

    Ok(ShellCommand {
        plain_command: tokens[0].clone(),
        args: tokens[1..].to_vec(),
        stdout_redirect,
        stderr_redirect,
    })
}
