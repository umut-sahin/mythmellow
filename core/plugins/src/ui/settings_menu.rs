use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        ui::settings_menu::*,
        utility::*,
    },
};

/// Plugin for managing the settings menu of the application.
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<SettingsMenu>();
        app.register_type::<SettingsMenuBackButton>();

        // Add systems.
        app.add_systems(OnEnter(InSettingsMenu), spawn_settings_menu);
        app.add_systems(
            PostUpdate,
            (
                previous_language_button_interaction,
                next_language_button_interaction,
                back_button_interaction,
            )
                .run_if(in_state(InSettingsMenu))
                .run_if(console_is_not_open),
        );
    }
}
