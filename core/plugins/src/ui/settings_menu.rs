use {
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::ui::settings_menu::*,
};

/// Plugin for managing the settings menu of the application.
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        // Add systems.
        app.add_systems(OnEnter(InSettingsMenu), spawn_settings_menu);
    }
}
