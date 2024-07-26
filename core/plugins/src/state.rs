use {
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_sets::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        state::*,
        utility::*,
    },
};

/// Plugin for managing the states of the application.
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<GameStateStack>();
        app.register_type::<GameResult>();

        // Register states.
        app.register_type::<AppState>();
        app.register_type::<GameState>();
        app.register_type::<InMenu>();
        app.register_type::<InGame>();
        app.register_type::<InChapter>();
        app.register_type::<LocalizationState>();
        app.register_type::<DiagnosticsOverlayState>();

        // Initialize states.
        app.init_state::<AppState>();
        app.add_sub_state::<GameState>();
        app.add_computed_state::<InMenu>();
        app.add_computed_state::<InSettingsMenu>();
        app.add_computed_state::<InGame>();
        app.add_computed_state::<InChapter>();
        app.init_state::<LocalizationState>();
        app.init_state::<DiagnosticsOverlayState>();

        // Enable state scoped entities.
        app.enable_state_scoped_entities::<AppState>();
        app.enable_state_scoped_entities::<GameState>();
        app.enable_state_scoped_entities::<InMenu>();
        app.enable_state_scoped_entities::<InSettingsMenu>();
        app.enable_state_scoped_entities::<InGame>();
        app.enable_state_scoped_entities::<InChapter>();

        // Insert resources.
        app.init_resource::<GameStateStack>();

        // Enable diagnostics overlay if it's enabled in the general settings.
        let general_settings = app.world_mut().resource::<Persistent<GeneralSettings>>();
        if general_settings.show_diagnostics_overlay {
            app.world_mut().insert_resource(NextState::Pending(DiagnosticsOverlayState::Enabled));
        }

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            start_loading.in_set(InitializationSystems::Done),
        );
        app.add_systems(OnEnter(GameState::Loading), start_playing.in_set(LoadingSystems::Done));
        app.add_systems(OnEnter(GameState::Restart), restart.in_set(RestartSystems::Done));
        app.add_systems(
            Update,
            game_state_transition
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Transition)),
        );
        app.add_systems(OnExit(AppState::Game), reset_resource::<GameStateStack>);
    }
}
