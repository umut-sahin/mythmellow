//! Command systems.

use crate::{
    prelude::*,
    systems::utility::*,
};


/// Handles the experience commands.
pub fn experience_command_handler(
    mut command: ConsoleCommand<ExperienceCommand>,
    player_query: Query<(Entity, &Experience), With<Player>>,
    mut experience_gained_event_writer: EventWriter<ExperienceGainedEvent>,
) {
    if let Some(Ok(ExperienceCommand { subcommand })) = command.take() {
        let (player_entity, player_experience) = if let Ok(query_result) = player_query.get_single()
        {
            query_result
        } else {
            reply!(command, "Not available outside the game.");
            reply!(command, "");
            return;
        };

        match subcommand {
            ExperienceCommands::Show => {
                reply!(command, "{}", *player_experience);
            },
            ExperienceCommands::Add { experience } => {
                experience_gained_event_writer.send(ExperienceGainedEvent {
                    entity: player_entity,
                    experience: Experience(experience),
                    by: "cheating :)".to_owned(),
                });
                reply!(command, "Added.");
            },
        }
        reply!(command, "");
    }
}

/// Handles the god mode commands.
pub fn god_mode_command_handler(
    mut command: ConsoleCommand<GodModeCommand>,
    mut god_mode: ResMut<GodMode>,
) {
    if let Some(Ok(GodModeCommand { subcommand })) = command.take() {
        match subcommand {
            GodModeCommands::Status => {
                let status = if god_mode.is_enabled { "Enabled" } else { "Disabled" };
                reply!(command, "{}.", status);
            },
            GodModeCommands::Enable => {
                if god_mode.is_enabled {
                    reply!(command, "Already enabled.");
                } else {
                    god_mode.is_enabled = true;
                    reply!(command, "Enabled.");
                }
            },
            GodModeCommands::Disable => {
                if god_mode.is_enabled {
                    god_mode.is_enabled = false;
                    reply!(command, "Disabled.");
                } else {
                    reply!(command, "Already disabled.");
                }
            },
        }
        reply!(command, "");
    }
}

/// Handles the inventory commands.
pub fn inventory_command_handler(
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

/// Handles the level commands.
pub fn level_command_handler(
    mut command: ConsoleCommand<LevelCommand>,
    mut player_query: Query<&Level, With<Player>>,
    registered_systems: Res<RegisteredSystems>,
    mut commands: Commands,
) {
    if let Some(Ok(LevelCommand { subcommand })) = command.take() {
        let player_level = if let Ok(query_result) = player_query.get_single_mut() {
            query_result
        } else {
            reply!(command, "Not available outside the game.");
            reply!(command, "");
            return;
        };

        match subcommand {
            LevelCommands::Show => {
                reply!(command, "{}", player_level.get());
            },
            LevelCommands::Set { level } => {
                commands.run_system_with_input(registered_systems.leveling.set_level, Level(level));
                reply!(command, "Set.");
            },
        }
        reply!(command, "");
    }
}

/// Handles the locale commands.
pub fn locale_command_handler(
    mut command: ConsoleCommand<LocaleCommand>,
    locale: ResMut<Locale>,
    registered_systems: Res<RegisteredSystems>,
    mut commands: Commands,
) {
    if let Some(Ok(LocaleCommand { subcommand })) = command.take() {
        match subcommand {
            LocaleCommands::Show => {
                reply!(command, "{}", locale.requested);
            },
            LocaleCommands::Set { locale: requested } => {
                match requested.parse::<LanguageIdentifier>() {
                    Ok(new_locale) => {
                        commands.run_system_with_input(
                            registered_systems.localization.set_locale,
                            new_locale,
                        );
                        reply!(command, "Set.");
                    },
                    Err(_) => {
                        reply!(command, "Requested locale isn't valid.");
                    },
                }
            },
        }
        reply!(command, "");
    }
}
