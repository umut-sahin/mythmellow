//! Experience commands.

use crate::prelude::*;

/// Controls the experience of the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "experience")]
#[command(disable_help_flag = true)]
pub struct ExperienceCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: ExperienceCommands,
}

/// Experience subcommands.
#[derive(Debug, Subcommand)]
pub enum ExperienceCommands {
    /// Show the experience of the player.
    Show,
    /// Increase the experience of the player.
    Add {
        /// Experience to add.
        experience: f64,
    },
}
