//! Plugins of the game.

mod action;
mod camera;
mod combat;
mod configuration;
mod console;
mod enemy;
mod inventory;
mod item;
mod leveling;
mod localization;
mod map;
mod mode;
mod movement;
mod physics;
mod player;
mod property;
mod set;
mod state;
mod ui;
mod utility;

pub use crate::{
    action::ActionPlugin,
    camera::CameraPlugin,
    combat::CombatPlugin,
    configuration::ConfigurationPlugin,
    console::ConsolePlugin,
    enemy::EnemyPlugin,
    inventory::InventoryPlugin,
    item::ItemPlugin,
    leveling::LevelingPlugin,
    localization::LocalizationPlugin,
    map::MapPlugin,
    mode::ModePlugin,
    movement::MovementPlugin,
    physics::PhysicsPlugin,
    player::PlayerPlugin,
    property::PropertyPlugin,
    set::SetPlugin,
    state::StatePlugin,
    ui::{
        DiagnosticsOverlayPlugin,
        EnemySelectionScreenPlugin,
        GameOverMenuPlugin,
        HudPlugin,
        MainMenuPlugin,
        PauseMenuPlugin,
        PlayerSelectionScreenPlugin,
        SettingsMenuPlugin,
        UiPlugin,
        WidgetPlugin,
    },
    utility::UtilityPlugin,
};
