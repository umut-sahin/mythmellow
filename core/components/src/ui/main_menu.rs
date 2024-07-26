//! Main menu components.

use mythmallow_core_dependencies::*;


/// Component for the main menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct MainMenu;


/// Component for the play button in the main menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct MainMenuPlayButton;


/// Component for the settings button in the main menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct MainMenuSettingsButton;


/// Component for the quit button in the main menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct MainMenuQuitButton;
