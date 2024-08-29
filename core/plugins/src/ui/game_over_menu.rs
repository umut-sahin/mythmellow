use {
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        ui::game_over_menu::*,
        utility::*,
    },
};

/// Plugin for managing the game over menu of the application.
pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        // Add systems.
        app.add_systems(OnEnter(GameState::Over), spawn_game_over_menu);
        app.add_systems(OnExit(GameState::Over), remove_resource::<GameResult>);
    }
}
