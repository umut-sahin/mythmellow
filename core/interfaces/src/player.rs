use {
    mythmallow_core_components::all::*,
    mythmallow_core_constants::player::*,
    mythmallow_core_dependencies::*,
};

/// Interface for the mythologies.
pub trait IMythology: Any + Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the mythology.
    fn id(&self) -> SmolStr;

    /// Gets the localized name of the mythology.
    fn name(&self) -> LocalizedText;
}

/// Interface for the players.
pub trait IPlayer: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the player.
    fn id(&self) -> SmolStr;

    /// Gets the localized name of the player.
    fn name(&self) -> LocalizedText;


    /// Gets the base dash cooldown duration of the player.
    fn dash_cooldown_duration(&self) -> DashCooldownDuration {
        DashCooldownDuration(BASE_PLAYER_DASH_COOLDOWN_DURATION)
    }

    /// Gets the base dash duration of the player.
    fn dash_duration(&self) -> DashDuration {
        DashDuration(BASE_PLAYER_DASH_DURATION)
    }

    /// Gets the base dash speed multiplier of the player.
    fn dash_speed_multiplier(&self) -> DashSpeedMultiplier {
        DashSpeedMultiplier::default()
    }

    /// Gets the base dodge chance of the player.
    fn dodge_chance(&self) -> DodgeChance {
        DodgeChance::default()
    }

    /// Gets the base health of the player.
    fn health(&self) -> Health {
        Health(BASE_PLAYER_HEALTH)
    }

    /// Gets the base speed of the player.
    fn speed(&self) -> Speed {
        Speed(BASE_PLAYER_SPEED)
    }

    /// Gets the base speed multiplier of the player.
    fn speed_multiplier(&self) -> SpeedMultiplier {
        SpeedMultiplier::default()
    }


    /// Gets the collider of the player.
    fn collider(&self) -> Collider;

    /// Spawns the player.
    fn spawn(&self, world: &mut World);
}
