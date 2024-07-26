//! Plugin of the `greek` items.

use crate::prelude::*;

/// Plugin for managing the `greek` items.
pub struct GreekItemsPlugin;

impl Plugin for GreekItemsPlugin {
    fn build(&self, app: &mut App) {
        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("greek-items.ftl");

        // Add item plugins.
        app.add_plugins(BowOfArtemisPlugin);
        app.add_plugins(BidentOfHadesPlugin);
    }
}
