use mythmallow_core_dependencies::*;

/// Actions that can be performed from anywhere.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum GlobalAction {
    /// Toggle the fullscreen mode.
    ToggleFullscreen,

    /// Toggle the diagnostics overlay.
    ToggleDiagnosticsOverlay,

    /// Toggle the physics gizmos.
    #[cfg(feature = "development")]
    TogglePhysicsGizmos,
}

impl GlobalAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default());

        // Create the input map.
        #[allow(unused_mut)]
        let mut input_map = InputMap::new([
            (GlobalAction::ToggleFullscreen, KeyCode::F11),
            (GlobalAction::ToggleDiagnosticsOverlay, KeyCode::F10),
        ]);
        #[cfg(feature = "development")]
        input_map.insert(
            GlobalAction::TogglePhysicsGizmos,
            ButtonlikeChord::new([KeyCode::ControlLeft, KeyCode::KeyP]),
        );

        // Insert the input map resource.
        app.insert_resource(input_map);

        // Insert the global action state as a resource.
        app.insert_resource(ActionState::<GlobalAction>::default());
    }
}
