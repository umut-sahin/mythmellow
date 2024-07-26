use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_sets::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::ui::hud::*,
};

/// Plugin for managing the head-up display of the game.
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Hud>();

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            spawn_hud.in_set(InitializationSystems::Hud),
        );
        app.add_systems(PostUpdate, update_player_health_bar.run_if(in_state(AppState::Game)));
        app.add_systems(
            PostUpdate,
            update_player_experience_bar
                .run_if(in_state(AppState::Game))
                .run_if(resource_exists::<ExperienceRequiredToGetToCurrentLevel>)
                .run_if(resource_exists::<ExperienceRequiredToLevelUp>),
        );
    }
}
