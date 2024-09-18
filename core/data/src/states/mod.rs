//! States of the game.

mod app;
mod diagnostics_overlay;
mod game;
mod in_chapter;
mod in_game;
mod in_menu;
mod in_settings_menu;
mod localization;

pub use {
    app::AppState,
    diagnostics_overlay::DiagnosticsOverlayState,
    game::GameState,
    in_chapter::InChapter,
    in_game::InGame,
    in_menu::InMenu,
    in_settings_menu::InSettingsMenu,
    localization::LocalizationState,
};
