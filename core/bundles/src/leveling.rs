//! Leveling bundles.

use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
};


/// Bundle for the experience points.
#[derive(Bundle)]
pub struct ExperiencePointBundle {
    /// Position of the experience point.
    pub position: Position,

    /// Attraction speed of the experience point.
    pub attraction_speed: AttractionSpeed,

    /// Mesh of the experience point.
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,

    /// Collider of the experience point.
    pub collider: Collider,

    /// Amount of experience in the experience point.
    pub experience: Experience,
}

impl ExperiencePointBundle {
    /// Spawns an experience point.
    pub fn spawn<'c>(
        self,
        commands: &'c mut Commands,
        counter: &mut ExperiencePointCounter,
    ) -> EntityCommands<'c> {
        counter.increment();
        commands.spawn((
            // Tags
            Name::new(format!("Experience Point {}", counter.get())),
            ExperiencePoint,
            // Experience Point
            self,
            // Physics
            RigidBody::Kinematic,
            Restitution::PERFECTLY_INELASTIC,
            LinearVelocity::ZERO,
            LockedAxes::ROTATION_LOCKED,
            CollisionLayers::new(
                [Layer::ExperiencePoint],
                [Layer::Player, Layer::PlayerPickupArea],
            ),
        ))
    }
}
