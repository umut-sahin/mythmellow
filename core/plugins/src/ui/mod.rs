use {
    mythmallow_core_assets::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
};

/// Plugin for managing the user interface of the application.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("ui.ftl");

        // Add materials.
        app.add_plugins(UiMaterialPlugin::<BarMaterial>::default());

        // Add sub plugins.
        app.add_plugins(WidgetPlugin);
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(SettingsMenuPlugin);
        app.add_plugins(GameModeSelectionScreenPlugin);
        app.add_plugins(PlayerSelectionScreenPlugin);
        app.add_plugins(EnemySelectionScreenPlugin);
        app.add_plugins(HudPlugin);
        app.add_plugins(PauseMenuPlugin);
        app.add_plugins(GameOverMenuPlugin);
        app.add_plugins(DiagnosticsOverlayPlugin);

        // Add sickle ui plugin.
        app.add_plugins(SickleUiPlugin);
    }
}

mod widget;
pub use widget::WidgetPlugin;

mod main_menu;
pub use main_menu::MainMenuPlugin;

mod settings_menu;
pub use settings_menu::SettingsMenuPlugin;

mod game_mode_selection_screen;
pub use game_mode_selection_screen::GameModeSelectionScreenPlugin;

mod player_selection_screen;
pub use player_selection_screen::PlayerSelectionScreenPlugin;

mod enemy_selection_screen;
pub use enemy_selection_screen::EnemySelectionScreenPlugin;

mod hud;
pub use hud::HudPlugin;

mod pause_menu;
pub use pause_menu::PauseMenuPlugin;

mod game_over_menu;
pub use game_over_menu::GameOverMenuPlugin;

mod diagnostics_overlay;
pub use diagnostics_overlay::DiagnosticsOverlayPlugin;
