use {
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_systems::utility::*,
};

/// Plugin for managing the utilities of the application.
pub struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) {
        // Initialize resources.
        RegisteredSystems::initialize(app);

        // Setup the random number generation.
        let seed = {
            let arguments = app.world_mut().resource::<Arguments>();
            let seed = arguments.seed.unwrap_or_else(|| WyRand::from_entropy().gen::<u64>());

            log::info!("seeding {}", seed);
            WyRand::seed_from_u64(seed).gen::<[u8; 8]>()
        };
        app.add_plugins(EntropyPlugin::<WyRand>::with_seed(seed));

        // Group unknown global entities.
        {
            let others = app.world_mut().spawn(Name::new("Others")).id();

            #[rustfmt::skip]
            app.add_systems(
                Update,
                move |
                    mut commands: Commands,
                    other_query: Query<Entity, (Without<Parent>, Without<Name>)>
                | {
                    for entity in other_query.iter() {
                        commands.entity(entity).set_parent(others);
                    }
                },
            );
        }
    }
}
