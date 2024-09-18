use crate::{
    prelude::*,
    systems::{
        physics::*,
        utility::*,
    },
};

/// Plugin for managing the physics of the game.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Register layers.
        app.register_type::<Layer>();

        // Setup easing.
        app.add_plugins(EasingsPlugin);

        // Setup physics.
        app.insert_resource(Gravity::ZERO);
        app.add_plugins(AvianPlugin::default());

        // Setup physics gizmos in development mode.
        #[cfg(feature = "development")]
        {
            use mythmallow_core_data::prelude::*;

            let general_settings = app.world().resource::<Persistent<GeneralSettings>>();
            app.insert_gizmo_config(
                PhysicsGizmos::default(),
                GizmoConfig { enabled: general_settings.enable_physics_gizmos, ..default() },
            );
            app.add_plugins(PhysicsDebugPlugin::default());
        }

        // Pause physics at startup.
        app.world_mut().resource_mut::<Time<Physics>>().pause();

        // Add systems.
        app.add_systems(OnEnter(GameState::Playing), resume_physics.run_if(console_is_not_open));
        app.add_systems(OnExit(GameState::Playing), pause_physics);
    }
}
