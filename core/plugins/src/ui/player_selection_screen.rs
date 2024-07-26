use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        ui::player_selection_screen::*,
        utility::*,
    },
};

/// Plugin for managing the player selection screen of the application.
pub struct PlayerSelectionScreenPlugin;

impl Plugin for PlayerSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<PlayerSelectionScreen>();
        app.register_type::<PlayerSelectionScreenPlayerButton>();
        app.register_type::<PlayerSelectionScreenBackButton>();

        // Add systems.
        app.add_systems(OnEnter(AppState::PlayerSelectionScreen), spawn_player_selection_screen);
        app.add_systems(
            PostUpdate,
            (player_button_interaction, back_button_interaction)
                .run_if(in_state(AppState::PlayerSelectionScreen))
                .run_if(console_is_not_open),
        );
    }
}
