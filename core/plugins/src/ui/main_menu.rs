use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        ui::main_menu::*,
        utility::*,
    },
};

/// Plugin for managing the main menu of the application.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<MainMenu>();
        app.register_type::<MainMenuPlayButton>();
        app.register_type::<MainMenuSettingsButton>();
        app.register_type::<MainMenuQuitButton>();

        // Add systems.
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(
            PostUpdate,
            (play_button_interaction, settings_button_interaction, quit_button_interaction)
                .run_if(in_state(AppState::MainMenu))
                .run_if(console_is_not_open),
        );
    }
}
