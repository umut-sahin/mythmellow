use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        ui::enemy_selection_screen::*,
        utility::*,
    },
};

/// Plugin for managing the enemy selection screen of the application.
pub struct EnemySelectionScreenPlugin;

impl Plugin for EnemySelectionScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<EnemySelectionScreen>();
        app.register_type::<EnemySelectionScreenEnemyButton>();
        app.register_type::<EnemySelectionScreenBackButton>();

        // Add systems.
        app.add_systems(OnEnter(AppState::EnemySelectionScreen), spawn_enemy_selection_screen);
        app.add_systems(
            PostUpdate,
            (enemy_button_interaction, back_button_interaction)
                .run_if(in_state(AppState::EnemySelectionScreen))
                .run_if(console_is_not_open),
        );
    }
}
