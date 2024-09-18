use crate::prelude::*;

/// Plugin for managing the properties of the entities of the game.
pub struct PropertyPlugin;

impl Plugin for PropertyPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Damage>();
        app.register_type::<DashCooldownDuration>();
        app.register_type::<DashDuration>();
        app.register_type::<DashSpeedMultiplier>();
        app.register_type::<DodgeChance>();
        app.register_type::<Health>();
        app.register_type::<Range>();
        app.register_type::<RemainingHealth>();
        app.register_type::<Speed>();
        app.register_type::<SpeedMultiplier>();
    }
}
