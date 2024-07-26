use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_events::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_sets::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::leveling::*,
};

/// Plugin for managing the leveling of the entities of the game.
pub struct LevelingPlugin;

impl Plugin for LevelingPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<ExperienceRequiredToGetToCurrentLevel>();
        app.register_type::<ExperienceRequiredToLevelUp>();

        // Register components.
        app.register_type::<Level>();
        app.register_type::<Experience>();

        // Add events.
        app.add_event::<ExperienceGainedEvent>();
        app.add_event::<LeveledUpEvent>();

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize_player_level_structure.in_set(InitializationSystems::Leveling),
        );
        app.add_systems(
            PostUpdate,
            (
                gain_player_experience,
                level_player_up.run_if(
                    |player_query: Query<(), (With<Player>, Changed<Experience>)>| {
                        !player_query.is_empty()
                    },
                ),
            )
                .chain(),
        );
        app.add_systems(OnExit(InGame), deinitialize_player_level_structure);
    }
}
