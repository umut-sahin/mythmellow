//! Level commands.

use crate::prelude::*;

/// Controls the level of the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "level")]
#[command(disable_help_flag = true)]
pub struct LevelCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: LevelCommands,
}

/// Level subcommands.
#[derive(Debug, Subcommand)]
pub enum LevelCommands {
    /// Show the level of the player.
    Show,
    /// Set the level of the player.
    Set {
        /// Level to set.
        level: NonZeroU16,
    },
}
