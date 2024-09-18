use crate::{
    prelude::*,
    systems::{
        command::*,
        console::*,
    },
};

/// Plugin for managing the console of the application.
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        // Add console plugin.
        app.add_plugins(BevyConsolePlugin);

        // Overwrite console configuration.
        app.insert_resource(ConsoleConfiguration {
            symbol: "\n> ".to_owned(),
            background_color: Color32::from_black_alpha(200),
            ..default()
        });

        // Add systems.
        app.add_systems(
            Update,
            set_console_size.run_if(
                |primary_window_query: Query<&PrimaryWindow, Changed<Window>>| {
                    !primary_window_query.is_empty()
                },
            ),
        );
        app.add_systems(
            Update,
            control_physics_time
                .run_if(|console_state: Res<ConsoleState>| console_state.is_changed()),
        );

        // Add commands.
        app.add_console_command::<ExperienceCommand, _>(experience_command_handler);
        app.add_console_command::<GodModeCommand, _>(god_mode_command_handler);
        app.add_console_command::<InventoryCommand, _>(inventory_command_handler);
        app.add_console_command::<LevelCommand, _>(level_command_handler);
        app.add_console_command::<LocaleCommand, _>(locale_command_handler);
    }
}
