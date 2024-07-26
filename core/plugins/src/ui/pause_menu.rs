use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        ui::pause_menu::*,
        utility::*,
    },
};

/// Plugin for managing the pause menu of the application.
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<PauseMenu>();
        app.register_type::<PauseMenuResumeButton>();
        app.register_type::<PauseMenuRestartButton>();
        app.register_type::<PauseMenuSettingsButton>();
        app.register_type::<PauseMenuReturnToMainMenuButton>();
        app.register_type::<PauseMenuQuitToDesktopButton>();

        // Add systems.
        app.add_systems(OnEnter(GameState::Paused), spawn_pause_menu);
        app.add_systems(
            PostUpdate,
            (
                resume_button_interaction,
                restart_button_interaction,
                settings_button_interaction,
                return_to_main_menu_button_interaction,
                quit_button_interaction,
            )
                .run_if(in_state(GameState::Paused))
                .run_if(console_is_not_open),
        );
    }
}
