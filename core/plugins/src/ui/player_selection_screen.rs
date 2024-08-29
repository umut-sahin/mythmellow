use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::ui::player_selection_screen::*,
};

/// Plugin for managing the player selection screen of the application.
pub struct PlayerSelectionScreenPlugin;

impl Plugin for PlayerSelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<PlayerSelectionScreenPlayerButton>();

        // Add systems.
        app.add_systems(OnEnter(AppState::PlayerSelectionScreen), spawn_player_selection_screen);
    }
}
