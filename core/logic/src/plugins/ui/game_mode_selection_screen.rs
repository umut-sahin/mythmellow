use crate::{
    prelude::*,
    systems::ui::game_mode_selection_screen::*,
};

/// Plugin for managing the game mode selection screen of the application.
pub struct GameModeSelectionScreenPlugin;

impl Plugin for GameModeSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<GameModeSelectionScreenGameModeButton>();

        // Add systems.
        app.add_systems(
            OnEnter(AppState::GameModeSelectionScreen),
            spawn_game_mode_selection_screen,
        );
    }
}
