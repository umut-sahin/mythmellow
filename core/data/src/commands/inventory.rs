//! Inventory commands.

use crate::prelude::*;

/// Controls the inventory of the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "inventory")]
#[command(disable_help_flag = true)]
pub struct InventoryCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: InventoryCommands,
}

/// Inventory subcommands.
#[derive(Debug, Subcommand)]
pub enum InventoryCommands {
    /// List the items in the inventory.
    List,
    /// Add an item to the inventory.
    Add {
        /// Item to add.
        item: String,
    },
}
