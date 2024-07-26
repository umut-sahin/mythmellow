//! Plugin of the `greek` players.

use crate::prelude::*;

/// Plugin for managing the `greek` players.
pub struct GreekPlayersPlugin;

impl Plugin for GreekPlayersPlugin {
    fn build(&self, app: &mut App) {
        // Setup localization.
        app.world_mut().resource_mut::<LocaleAssets>().push("greek-players.ftl");

        // Add player plugins.
        app.add_plugins(ArtemisPlugin);
        app.add_plugins(HadesPlugin);
    }
}
