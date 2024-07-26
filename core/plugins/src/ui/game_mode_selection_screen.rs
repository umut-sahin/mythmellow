use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        ui::game_mode_selection_screen::*,
        utility::*,
    },
};

/// Plugin for managing the game mode selection screen of the application.
pub struct GameModeSelectionScreenPlugin;

impl Plugin for GameModeSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<GameModeSelectionScreen>();
        app.register_type::<GameModeSelectionScreenGameModeButton>();
        app.register_type::<GameModeSelectionScreenBackButton>();

        // Add systems.
        app.add_systems(
            OnEnter(AppState::GameModeSelectionScreen),
            spawn_game_mode_selection_screen,
        );
        app.add_systems(
            PostUpdate,
            (game_mode_button_interaction, back_button_interaction)
                .run_if(in_state(AppState::GameModeSelectionScreen))
                .run_if(console_is_not_open),
        );
    }
}
