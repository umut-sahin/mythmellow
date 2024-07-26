use {
    mythmallow_core_actions::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_sets::*,
    mythmallow_core_systems::action::*,
};

/// Plugin for managing the actions of the application.
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        // Setup actions.
        GlobalAction::setup(app);
        MenuAction::setup(app);
        GameAction::setup(app);

        // Add systems.
        {
            app.add_systems(Update, toggle_fullscreen);
            app.add_systems(Update, toggle_diagnostics_overlay);
            #[cfg(feature = "development")]
            app.add_systems(Update, toggle_physics_gizmos);

            app.add_systems(PostUpdate, pause.in_set(GameplaySystems::Player));
        }
    }
}
