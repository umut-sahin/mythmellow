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
    mythmallow_core_data::{
        constants::enemy::*,
        prelude::*,
    },
    mythmallow_core_dependencies::prelude::*,
    mythmallow_core_logic::systems::utility::*,
};
