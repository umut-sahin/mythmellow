use {
    crate::{
        prelude::*,
        systems::camera::*,
    },
    mythmallow_core_data::constants::camera::*,
};

/// Plugin for managing the cameras of the application.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<MainCamera>();

        // Set the background color of the application.
        app.insert_resource(ClearColor(BACKGROUND_COLOR));

        // Add systems.
        {
            app.add_systems(Startup, spawn_main_camera);

            app.add_systems(
                OnEnter(GameState::Loading),
                player_lock.in_set(LoadingSystems::Camera),
            );
            app.add_systems(
                PostUpdate,
                player_lock
                    .in_set(GameplaySystems::Camera)
                    .before(TransformSystem::TransformPropagate)
                    .after(PhysicsSet::Sync),
            );

            #[cfg(feature = "development")]
            app.add_systems(
                PostUpdate,
                set_default_ui_camera.run_if(|editor: Res<Editor>| editor.is_changed()),
            );
        }
    }
}
