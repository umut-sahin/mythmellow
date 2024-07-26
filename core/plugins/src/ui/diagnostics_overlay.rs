use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::ui::diagnostics_overlay::*,
};

/// Plugin for managing the diagnostics overlay of the application.
pub struct DiagnosticsOverlayPlugin;

impl Plugin for DiagnosticsOverlayPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<DiagnosticsOverlayText>();

        // Add systems.
        app.add_systems(OnEnter(DiagnosticsOverlayState::Enabled), spawn_diagnostics_overlay);
        app.add_systems(
            Update,
            update_diagnostics_overlay.run_if(in_state(DiagnosticsOverlayState::Enabled)),
        );
        app.add_systems(OnExit(DiagnosticsOverlayState::Enabled), despawn_diagnostics_overlay);
    }
}
