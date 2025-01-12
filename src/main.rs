//! Rusty Shell implementation
//!
//! This is the main entry point for the shell.
//! It handles the main REPL and
//! coordinates between different modules.
//!
//! @author: @grainme

mod builtins;
mod command;
mod environment;
mod error;
mod parser;

fn main() {
    command::run();
}
