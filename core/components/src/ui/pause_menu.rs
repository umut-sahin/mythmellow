//! Pause menu components.

use mythmallow_core_dependencies::*;


/// Component for the pause menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct PauseMenu;


/// Component for the resume button in the pause menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct PauseMenuResumeButton;


/// Component for the restart button in the pause menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct PauseMenuRestartButton;


/// Component for the settings button in the pause menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct PauseMenuSettingsButton;


/// Component for the return to main menu  button in the pause menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct PauseMenuReturnToMainMenuButton;


/// Component for the quit to desktop button in the pause menu.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct PauseMenuQuitToDesktopButton;
