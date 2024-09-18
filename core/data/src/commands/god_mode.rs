//! God mode commands.

use crate::prelude::*;

/// Controls the god mode of the game.
#[derive(ConsoleCommand, Parser)]
#[command(name = "god-mode")]
#[command(disable_help_flag = true)]
pub struct GodModeCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: GodModeCommands,
}

/// God mode subcommands.
#[derive(Debug, Subcommand)]
pub enum GodModeCommands {
    /// Shows the status of the god mode.
    Status,
    /// Enables the god mode.
    Enable,
    /// Disables the god mode.
    Disable,
}
