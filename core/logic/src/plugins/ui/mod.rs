use crate::prelude::*;

mod diagnostics_overlay;
mod enemy_selection_screen;
mod game_mode_selection_screen;
mod game_over_menu;
mod hud;
mod main_menu;
mod market_screen;
mod pause_menu;
mod player_selection_screen;
mod settings_menu;
mod widget;

pub use {
    diagnostics_overlay::*,
    enemy_selection_screen::*,
    game_mode_selection_screen::*,
    game_over_menu::*,
    hud::*,
    main_menu::*,
    market_screen::*,
    pause_menu::*,
    player_selection_screen::*,
    settings_menu::*,
    widget::*,
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
        app.add_plugins(MarketScreenPlugin);
        app.add_plugins(GameOverMenuPlugin);
        app.add_plugins(DiagnosticsOverlayPlugin);

        // Add sickle ui plugin.
        app.add_plugins(SickleUiPlugin);
    }
}
