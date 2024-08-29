use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
};

/// Controls the inventory of the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "inventory")]
#[command(disable_help_flag = true)]
pub struct InventoryCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: InventoryCommands,
}

impl InventoryCommand {
    /// Handles the command.
    pub fn handler(
        mut command: ConsoleCommand<InventoryCommand>,
        player_query: Query<Entity, With<Player>>,
        mut inventory: ResMut<Inventory>,
        item_registry: Res<ItemRegistry>,
    ) {
        if let Some(Ok(InventoryCommand { subcommand })) = command.take() {
            if player_query.is_empty() {
                reply!(command, "Not available outside the game.");
                reply!(command, "");
                return;
            }

            match subcommand {
                InventoryCommands::List => {
                    if inventory.is_empty() {
                        reply!(command, "Inventory is empty.");
                    } else {
                        for (i, item) in inventory.iter().enumerate() {
                            reply!(command, "{}) {}", i + 1, item.id());
                        }
                    }
                },
                InventoryCommands::Add { item } => {
                    match item_registry.find_item_by_id(&item) {
                        Some(item) => {
                            inventory.add(item.instantiate());
                            reply!(command, "Added.");
                        },
                        None => {
                            reply!(
                                command,
                                "Failed to add {:?} to the inventory as it doesn't exist.",
                                item,
                            );
                            reply!(command, "Run \"item list\" to see available items.")
                        },
                    }
                },
            }
            reply!(command, "");
        }
    }
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
