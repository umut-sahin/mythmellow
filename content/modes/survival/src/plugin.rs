//! Plugin of the `survival` mode.

use crate::{
    prelude::*,
    systems::*,
};

/// Plugin for managing the `survival` mode.
pub struct SurvivalModePlugin;

impl Plugin for SurvivalModePlugin {
    fn build(&self, app: &mut App) {
        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("survival-mode.ftl");

        // Register resources.
        app.register_type::<SurvivalModeArguments>();
        app.register_type::<GameMode<Survival>>();
        app.register_type::<CurrentWave>();
        app.register_type::<WaveTimer>();
        app.register_type::<WaveDurations>();

        let arguments = app.world().resource::<Arguments>();
        if let Some(game_mode_id_and_args) = &arguments.start_in_game_mode {
            let mut split = game_mode_id_and_args.split(' ');

            let game_mode = split.next().unwrap();
            let arguments = split;

            if game_mode == Survival.id().as_str() {
                let survival_mode_arguments =
                    SurvivalModeArguments::parse(std::iter::once(game_mode).chain(arguments));
                app.insert_resource(survival_mode_arguments);
            }
        }

        // Register the game mode.
        let mut game_mode_registry = app.world_mut().resource_mut::<GameModeRegistry>();
        game_mode_registry.register(Survival);

        // Add initialization systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize
                .in_set(InitializationSystems::GameMode)
                .run_if(resource_exists::<GameMode<Survival>>),
        );
        app.add_systems(
            OnEnter(GameState::Initialization),
            select_wave_when_starting_in_game
                .in_set(InitializationSystems::GameMode)
                .run_if(resource_exists::<GameMode<Survival>>)
                .run_if(resource_exists::<SurvivalModeArguments>)
                .after(initialize)
                .run_if(run_once()),
        );

        // Add loading systems.
        app.add_systems(
            OnEnter(GameState::Loading),
            (load, spawn_map)
                .chain()
                .in_set(LoadingSystems::GameMode)
                .run_if(resource_exists::<GameMode<Survival>>),
        );

        // Add gameplay systems.
        app.add_systems(
            PreUpdate,
            tick.in_set(GameplaySystems::GameMode).run_if(resource_exists::<GameMode<Survival>>),
        );

        // Add game won systems.
        app.add_systems(OnEnter(GameState::Won), win.run_if(resource_exists::<GameMode<Survival>>));

        // Add chapter ending systems.
        app.add_systems(OnExit(InChapter), unload.run_if(resource_exists::<GameMode<Survival>>));

        // Add game ending systems.
        app.add_systems(OnExit(InGame), deinitialize.run_if(resource_exists::<GameMode<Survival>>));
    }
}
