use {
    mythmallow_core_dependencies::*,
    mythmallow_core_systems::utility::*,
};

/// Controls the locale of the application.
#[derive(ConsoleCommand, Parser)]
#[command(name = "locale")]
#[command(disable_help_flag = true)]
pub struct LocaleCommand {
    /// Invoked subcommand.
    #[clap(subcommand)]
    pub subcommand: LocaleCommands,
}

impl LocaleCommand {
    /// Handles the command.
    pub fn handler(
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
