//! Prelude of the `sweet` enemies.

pub use crate::{
    chocolate_bar::ChocolateBar,
    gummy_bear::GummyBear,
    pack::SweetEnemyPack,
    plugin::SweetEnemiesPlugin,
};

pub(crate) use {
    crate::{
        chocolate_bar::ChocolateBarPlugin,
        gummy_bear::GummyBearPlugin,
    },
    mythmallow_core_bundles::all::*,
    mythmallow_core_components::all::*,
    mythmallow_core_constants::enemy::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_interfaces::*,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_sets::*,
    mythmallow_core_systems::utility::find_obstacle,
};
