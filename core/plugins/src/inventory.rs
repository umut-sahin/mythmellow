use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::inventory::*,
};

/// Plugin for managing the inventory of the player.
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<BaseOrientation>();

        // Insert resources.
        app.init_resource::<Inventory>();

        // Add systems.
        app.add_systems(
            Last,
            (
                acquire_release_items.run_if(|inventory: Res<Inventory>| inventory.is_changed()),
                reposition_weapons.run_if(
                    |weapon_query: Query<Entity, Added<Weapon>>,
                     player_query: Query<&Collider, (With<Player>, Changed<Collider>)>| {
                        !weapon_query.is_empty() || !player_query.is_empty()
                    },
                ),
                orient_weapons.after(reposition_weapons),
            ),
        );
        app.add_systems(OnExit(InGame), clear_inventory);
    }
}
