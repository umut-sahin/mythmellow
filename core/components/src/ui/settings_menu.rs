//! Settings menu components.

use mythmallow_core_dependencies::*;


/// Component for the settings menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct SettingsMenu;


/// Component for the previous language button in the settings menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct SettingsMenuPreviousLanguageButton;


/// Component for the next language button in the settings menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct SettingsMenuNextLanguageButton;


/// Component for the back button in the settings menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct SettingsMenuBackButton;
