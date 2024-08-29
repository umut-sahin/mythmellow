use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_events::all::*,
};

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
