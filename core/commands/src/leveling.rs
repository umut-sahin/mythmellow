use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_events::all::*,
    mythmallow_core_systems::utility::*,
};


/// Controls the level of the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "level")]
#[command(disable_help_flag = true)]
pub struct LevelCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: LevelCommands,
}

impl LevelCommand {
    /// Handles the command.
    pub fn handler(
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
                    commands
                        .run_system_with_input(registered_systems.leveling.set_level, Level(level));
                    reply!(command, "Set.");
                },
            }
            reply!(command, "");
        }
    }
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


/// Controls the experience of the player.
#[derive(ConsoleCommand, Parser)]
#[command(name = "experience")]
#[command(disable_help_flag = true)]
pub struct ExperienceCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: ExperienceCommands,
}

impl ExperienceCommand {
    /// Handles the command.
    pub fn handler(
        mut command: ConsoleCommand<ExperienceCommand>,
        player_query: Query<(Entity, &Experience), With<Player>>,
        mut experience_gained_event_writer: EventWriter<ExperienceGainedEvent>,
    ) {
        if let Some(Ok(ExperienceCommand { subcommand })) = command.take() {
            let (player_entity, player_experience) =
                if let Ok(query_result) = player_query.get_single() {
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
