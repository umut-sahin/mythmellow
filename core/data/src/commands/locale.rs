//! Locale commands.

use crate::prelude::*;

/// Controls the locale of the application.
#[derive(ConsoleCommand, Parser)]
#[command(name = "locale")]
#[command(disable_help_flag = true)]
pub struct LocaleCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: LocaleCommands,
}

/// Locale subcommands.
#[derive(Debug, Subcommand)]
pub enum LocaleCommands {
    /// Show the locale of the application.
    Show,
    /// Set the locale of the application.
    Set {
        /// New locale.
        locale: String,
    },
}
