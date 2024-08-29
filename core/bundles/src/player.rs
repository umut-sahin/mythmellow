//! Player bundles.

use {
    mythmallow_core_actions::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_interfaces::*,
    mythmallow_core_states::*,
};


/// Bundle for the player.
#[derive(Bundle, TypedBuilder)]
pub struct PlayerBundle<P: IPlayer + Component> {
    /// Player.
    pub player: P,

    /// Mesh of the player.
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,

    /// Input manager of the player.
    #[builder(setter(transform =
        |input_map: InputMap<GameAction>| {
            InputManagerBundle::<GameAction> { action_state: ActionState::default(), input_map }
        }
    ))]
    pub input: InputManagerBundle<GameAction>,
}

impl<P: IPlayer + Component> PlayerBundle<P> {
    /// Spawns the player.
    pub fn spawn<'c>(self, commands: &'c mut Commands) -> EntityCommands<'c> {
        let name = format!("Player [{}]", self.player.id());
        let collider = self.player.collider();

        let dash_cooldown_duration = self.player.dash_cooldown_duration();
        let dash_duration = self.player.dash_duration();
        let dash_speed_multiplier = self.player.dash_speed_multiplier();
        let dodge_chance = self.player.dodge_chance();
        let health = self.player.health();
        let pickup_range = self.player.pickup_range();
        let speed = self.player.speed();
        let speed_multiplier = self.player.speed_multiplier();

        let mut player = commands.spawn((
            // Tags
            Name::new(name),
            Player,
            // Player
            self,
            dash_cooldown_duration,
            dash_duration,
            dash_speed_multiplier,
            dodge_chance,
            health,
            speed,
            speed_multiplier,
            RemainingHealth(health.0),
            // Leveling
            Level::default(),
            Experience::default(),
            // Physics
            (
                RigidBody::Dynamic,
                LinearVelocity::ZERO,
                Restitution::PERFECTLY_INELASTIC,
                LockedAxes::ROTATION_LOCKED,
                collider.clone(),
                CollisionLayers::new([Layer::Player], [Layer::MapBound, Layer::ExperiencePoint]),
            ),
            // Lifetime
            StateScoped(InGame),
        ));

        player.with_children(|parent| {
            parent.spawn(PlayerHitBox::bundle(collider));
            parent.spawn(PlayerPickupArea::bundle(Collider::circle(*pickup_range)));
        });

        player
    }
}
