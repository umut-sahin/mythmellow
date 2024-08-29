use {
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::ui::main_menu::*,
};

/// Plugin for managing the main menu of the application.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Add systems.
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
    }
}
