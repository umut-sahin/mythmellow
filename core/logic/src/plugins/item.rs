use crate::prelude::*;

/// Plugin for managing the items of the game.
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Item>();
        app.register_type::<Weapon>();

        // Initialize registry.
        app.init_resource::<ItemRegistry>();
    }
}
