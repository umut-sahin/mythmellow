use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::ui::enemy_selection_screen::*,
};

/// Plugin for managing the enemy selection screen of the application.
pub struct EnemySelectionScreenPlugin;

impl Plugin for EnemySelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<EnemySelectionScreenEnemyButton>();

        // Add systems.
        app.add_systems(OnEnter(AppState::EnemySelectionScreen), spawn_enemy_selection_screen);
    }
}
