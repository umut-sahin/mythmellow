use {
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
};

/// Controls the god mode of the game.
#[derive(ConsoleCommand, Parser)]
#[command(name = "god-mode")]
#[command(disable_help_flag = true)]
pub struct GodModeCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: GodModeCommands,
}

impl GodModeCommand {
    /// Handles the command.
    pub fn handler(mut command: ConsoleCommand<GodModeCommand>, mut god_mode: ResMut<GodMode>) {
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
