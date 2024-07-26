//! Plugin of the `sweet` enemies.

use crate::prelude::*;

/// Plugin for managing the `sweet` enemies.
pub struct SweetEnemiesPlugin;

impl Plugin for SweetEnemiesPlugin {
    fn build(&self, app: &mut App) {
        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("sweet-enemies.ftl");

        // Add enemy plugins.
        app.add_plugins(GummyBearPlugin);
        app.add_plugins(ChocolateBarPlugin);
    }
}
