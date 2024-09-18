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
mod market;
mod mode;
mod movement;
mod physics;
mod player;
mod property;
mod set;
mod state;
mod ui;
mod utility;

pub use {
    action::*,
    camera::*,
    combat::*,
    configuration::*,
    console::*,
    enemy::*,
    inventory::*,
    item::*,
    leveling::*,
    localization::*,
    map::*,
    market::*,
    mode::*,
    movement::*,
    physics::*,
    player::*,
    property::*,
    set::*,
    state::*,
    ui::*,
    utility::*,
};
