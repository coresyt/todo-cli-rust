mod add;
mod list;

use clap::{ArgMatches, Command};

pub use add::AddCommand;
pub use list::ListCommand;
/// ## Trait template for CLI command implementations
///
/// `BasicCommand` is intended to be used as a base trait for implementing
/// individual commands in a command-line interface (CLI) application.
///
/// It enforces a common structure and behavior across all command implementations,
/// including instantiation, CLI argument definition, and execution logic.
pub trait BasicCommand {
    /// Creates a new instance of the implementing command struct.
    ///
    /// This method should return a new instance with any necessary default configuration.
    ///
    /// # Example
    /// ```rust
    /// let cmd = MyCommand::new();
    /// ```
    fn new() -> Self;

    /// Defines and returns the CLI structure of the command.
    ///
    /// This is where the command name, arguments, subcommands, and help message
    /// should be configured using a CLI argument parser (commonly `clap::Command`).
    ///
    /// # Returns
    /// A `Command` definition used by the CLI application to register this command.
    fn create_basic_command(&self) -> Command;

    /// Executes the command’s logic based on parsed CLI arguments.
    ///
    /// # Parameters
    /// - `path`: The working directory or base path relevant to the command execution.
    /// - `matches`: A reference to the parsed arguments provided by the CLI.
    ///
    /// This method should contain the main behavior of the command.
    fn code_to_exec(&self, path: String, matches: &ArgMatches);
}

