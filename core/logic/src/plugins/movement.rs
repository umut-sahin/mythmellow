use crate::{
    prelude::*,
    systems::{
        movement::*,
        utility::*,
    },
};

/// Plugin for managing the movement mechanics of the game.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<AttractedTo>();
        app.register_type::<AttractionSpeed>();
        app.register_type::<IdealAttractionDistance>();
        app.register_type::<SlowdownOfGoingBackwardsDuringAttraction>();

        // Add systems.
        app.add_systems(
            PreUpdate,
            (keep_dashing, cooldown::<Dashing>).in_set(GameplaySystems::Movement),
        );
        app.add_systems(
            Update,
            (attraction, (player_dash, player_movement).chain()).in_set(GameplaySystems::Movement),
        );
        app.add_systems(PostUpdate, start_dashing.in_set(GameplaySystems::Movement));
    }
}
