use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_registries::all::*,
};

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
