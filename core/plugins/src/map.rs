use {
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        map::*,
        utility::*,
    },
};

/// Plugin for managing the map of the game.
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<MapBounds>();

        // Add systems.
        app.add_systems(OnExit(GameState::Loading), spawn_map_bounds);
        app.add_systems(OnExit(InChapter), remove_resource::<MapBounds>);
    }
}
