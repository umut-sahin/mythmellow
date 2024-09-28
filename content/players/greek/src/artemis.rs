//! Implementation of the `artemis` player.

use crate::prelude::*;

/// Size of the player.
pub const SIZE: f32 = 20.00;

/// Color of the player.
pub const COLOR: Color = Color::srgb(0.00, 1.00, 0.00);

/// Component for the `artemis` player.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct Artemis;

impl IPlayer for Artemis {
    fn id(&self) -> SmolStr {
        "artemis".into()
    }

    fn name(&self) -> LocalizedText {
        LocalizedText::Localized {
            key: "artemis-name",
            args: smallvec![],
            fallback: "Artemis".into(),
        }
    }

    fn collider(&self) -> Collider {
        Collider::circle(SIZE)
    }

    fn spawn(&self, world: &mut World) {
        world.run_system_once_with(self.clone(), spawn);
    }
}

/// Plugin for managing the player.
pub struct ArtemisPlugin;

impl Plugin for ArtemisPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Artemis>();

        // Register the player.
        let mut player_registry = app.world_mut().resource_mut::<PlayerRegistry>();
        player_registry.register(GreekMythology, Artemis);
    }
}

/// Spawns the player.
pub fn spawn(
    In(player): In<Artemis>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_action_input_map: Res<InputMap<GameAction>>,
    mut inventory: ResMut<Inventory>,
) {
    let mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(SIZE)).into(),
        material: materials.add(ColorMaterial::from(COLOR)),
        transform: Transform::from_translation(Vec3::new(0.00, 0.00, Depth::Player.z())),
        ..default()
    };

    PlayerBundle::builder()
        .player(player)
        .mesh(mesh)
        .input(game_action_input_map.clone())
        .build()
        .spawn(&mut commands);

    inventory.add(BowOfArtemis.instantiate());
}