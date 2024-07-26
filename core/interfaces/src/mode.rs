use {
    crate::*,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
};

/// Interface for the game modes.
pub trait IGameMode: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the game mode.
    fn id(&self) -> SmolStr;

    /// Gets the localized name of the game mode.
    fn name(&self) -> LocalizedText;


    /// Gets the player level structure of the game mode.
    fn player_level_structure(&self) -> PlayerLevelStructure;

    /// Gets the default enemy spawn pattern of the game mode.
    fn default_enemy_spawn_pattern(&self, world: &World) -> EnemySpawnPattern;


    /// Initializes the game mode.
    fn initialize(&self, world: &mut World);

    /// Deinitializes the game mode.
    fn deinitialize(&self, world: &mut World);
}


/// Resource for the level structure of the player for the selected game mode.
#[derive(Clone, Resource)]
pub struct PlayerLevelStructure {
    /// Max level that can be reached in the game mode.
    pub max_level: Option<Level>,
    /// Function to calculate the required experience for each level.
    pub required_experience_calculator: fn(&World, Level) -> Experience,
}
