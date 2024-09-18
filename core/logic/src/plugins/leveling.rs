use crate::{
    prelude::*,
    systems::{
        leveling::*,
        utility::remove_resource,
    },
};

/// Plugin for managing the leveling of the entities of the game.
pub struct LevelingPlugin;

impl Plugin for LevelingPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<ExperienceRequiredToGetToCurrentLevel>();
        app.register_type::<ExperienceRequiredToLevelUp>();
        app.register_type::<ExperiencePointCounter>();

        // Register components.
        app.register_type::<Level>();
        app.register_type::<Experience>();
        app.register_type::<ExperiencePoint>();
        app.register_type::<ExperiencePointVisuals>();
        app.register_type::<ExperiencePointAttractionSpeed>();

        // Add events.
        app.add_event::<ExperienceGainedEvent>();
        app.add_event::<LeveledUpEvent>();

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize_player_level_structure.in_set(InitializationSystems::Leveling),
        );
        app.add_systems(
            OnEnter(GameState::Loading),
            initialize_experience_point_counter.in_set(LoadingSystems::Leveling),
        );
        app.add_systems(
            Update,
            (attract_experience_points, collect_experience_points)
                .chain()
                .in_set(GameplaySystems::Leveling),
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
        app.add_systems(OnExit(InChapter), remove_resource::<ExperiencePointCounter>);
        app.add_systems(
            OnExit(InGame),
            (
                remove_resource::<ExperienceRequiredToGetToCurrentLevel>,
                remove_resource::<ExperienceRequiredToLevelUp>,
                remove_resource::<PlayerLevelStructure>,
            ),
        );
    }
}
