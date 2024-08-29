use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
};

/// Interface for the enemy packs.
pub trait IEnemyPack: Any + Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the enemy pack.
    fn id(&self) -> SmolStr;

    /// Gets the localized name of the enemy pack.
    fn name(&self) -> LocalizedText;

    /// Gets the spawn pattern of the enemy pack,
    #[allow(unused_variables)]
    fn spawn_pattern(&self, world: &World) -> Option<EnemySpawnPattern> {
        None
    }
}

/// Interface for the enemies.
pub trait IEnemy: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the enemy.
    fn id(&self) -> SmolStr;

    /// Gets the localized name of the enemy.
    fn name(&self) -> LocalizedText;


    /// Gets the contact damage of the enemy.
    fn contact_damage(&self) -> Option<(Damage, DamageCooldown)> {
        None
    }

    /// Gets the health of the enemy.
    fn health(&self) -> Health;

    /// Gets the speed of the enemy.
    fn speed(&self) -> Speed;


    /// Gets the experience reward for defeating the enemy.
    fn experience_reward(&self) -> Experience;

    /// Gets the visuals of experience point dropped from the enemy.
    fn experience_point_visuals(&self) -> ExperiencePointVisuals {
        ExperiencePointVisuals::default()
    }

    /// Gets the speed of the experience point dropped from the enemy when they are attracted.
    fn experience_point_attraction_speed(&self) -> ExperiencePointAttractionSpeed {
        ExperiencePointAttractionSpeed::default()
    }


    /// Gets the collider of the enemy.
    fn collider(&self) -> Collider;

    /// Spawns the enemy.
    fn spawn(&self, world: &mut World, position: Position);
}


/// Resource for the enemy spawn pattern for the selected game mode and the selected enemy pack.
#[derive(Clone, Resource)]
pub struct EnemySpawnPattern {
    /// Spawns in the spawn pattern.
    pub spawns: Arc<Mutex<Vec<EnemySpawn>>>,
}

impl EnemySpawnPattern {
    /// Creates a new enemy spawn pattern.
    pub fn new(spawns: Vec<EnemySpawn>) -> EnemySpawnPattern {
        EnemySpawnPattern { spawns: Arc::new(Mutex::new(spawns)) }
    }
}

impl Debug for EnemySpawnPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for (i, spawn) in self.spawns.lock().unwrap().iter().enumerate() {
            map.entry(&i, &spawn);
        }
        map.finish()
    }
}


/// Container for the enemy spawns.
pub struct EnemySpawn {
    /// Delay for the first spawn.
    pub delay: Timer,
    /// Enemy to spawn.
    pub enemy: Arc<dyn IEnemy>,
    /// Group size.
    pub count: u32,
    /// Optional spawn interval within the group.
    pub interval: Option<Timer>,
    /// Group position.
    pub position: EnemySpawnPosition,
    /// Group direction.
    pub direction: EnemySpawnDirection,
    /// Group spread.
    pub spread: EnemySpawnSpread,
    /// Optional repeat for the spawn.
    pub repeat: Option<Timer>,

    /// Amount of enemies spawned within the group.
    ///
    /// Also used for retrying a failed spawn.
    pub spawned: u32,
    /// Amount of enemies to be spawned within the group.
    ///
    /// Can be used for multiple groups.
    pub remaining: u32,
    /// Current spawn position for the group.
    ///
    /// Updated on every `self.count` spawns.
    pub group_position: Position,
}

impl EnemySpawn {
    /// Creates a new enemy spawn.
    pub fn new(delay: Duration, enemy: impl IEnemy) -> EnemySpawn {
        EnemySpawn {
            delay: Timer::new(delay, TimerMode::Once),
            enemy: Arc::new(enemy),
            count: 1,
            interval: None,
            position: EnemySpawnPosition::Random,
            direction: EnemySpawnDirection::any(),
            spread: EnemySpawnSpread::default(),
            repeat: None,
            spawned: 0,
            remaining: 0,
            group_position: Position::new(Vector::ZERO),
        }
    }

    /// Creates a new enemy spawn from a dynamic enemy.
    pub fn new_dyn(delay: Duration, enemy: &Arc<dyn IEnemy>) -> EnemySpawn {
        EnemySpawn {
            delay: Timer::new(delay, TimerMode::Once),
            enemy: Arc::clone(enemy),
            count: 1,
            interval: None,
            position: EnemySpawnPosition::Random,
            direction: EnemySpawnDirection::any(),
            spread: EnemySpawnSpread::default(),
            repeat: None,
            spawned: 0,
            remaining: 0,
            group_position: Position::new(Vector::ZERO),
        }
    }

    /// Sets the group size of the spawn.
    ///
    /// # Panics
    ///
    /// - Panics if `count` is zero.
    pub fn count(mut self, count: u32) -> EnemySpawn {
        if count == 0 {
            panic!("spawn count cannot be 0");
        }
        self.count = count;
        self
    }

    /// Sets the interval of the spawn.
    pub fn interval(mut self, interval: Duration) -> EnemySpawn {
        self.interval = Some(Timer::new(interval, TimerMode::Repeating));
        self
    }

    /// Sets the group position of the spawn.
    pub fn position(mut self, position: EnemySpawnPosition) -> EnemySpawn {
        self.position = position;
        self
    }

    /// Sets the group direction of the spawn.
    pub fn direction(mut self, direction: EnemySpawnDirection) -> EnemySpawn {
        self.direction = direction;
        self
    }

    /// Sets the group spread of the spawn.
    pub fn spread(mut self, spread: EnemySpawnSpread) -> EnemySpawn {
        self.spread = spread;
        self
    }

    /// Sets the repeat of the spawn.
    pub fn repeat(mut self, repeat: Duration) -> EnemySpawn {
        self.repeat = Some(Timer::new(repeat, TimerMode::Repeating));
        self
    }
}

impl Debug for EnemySpawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Pattern");
        s.field("delay", &self.delay.duration());
        s.field("enemy", &self.enemy);
        s.field("count", &self.count);
        if let Some(timer) = self.interval.as_ref() {
            s.field("interval", &timer.duration());
        }
        s.field("position", &self.position);
        s.field("direction", &self.direction);
        s.field("spread", &self.spread);
        if let Some(timer) = self.repeat.as_ref() {
            s.field("repeat", &timer.duration());
        }
        s.finish()
    }
}


/// Position of the enemy spawn.
#[derive(Clone, Copy, Debug)]
pub enum EnemySpawnPosition {
    /// In a predefined position. If set, spawn direction is ignored.
    At(Position),
    /// Within certain distance to the player.
    AroundPlayer {
        /// Minimum distance to the player.
        near: f32,
        /// Maximum distance to the player.
        far: f32,
    },
    /// Random across the whole map.
    Random,
}


/// Direction of the enemy spawn.
#[derive(Clone, Copy, Debug)]
pub struct EnemySpawnDirection {
    /// Degree to start from.
    pub from_degrees: f32,
    /// Degree to end at.
    pub to_degrees: f32,
}

impl EnemySpawnDirection {
    /// Gets all directions.
    pub fn any() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 0.00, to_degrees: 360.00 }
    }

    /// Gets the top direction.
    pub fn top() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 0.00, to_degrees: 180.00 }
    }

    /// Gets the bottom direction.
    pub fn bottom() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 180.00, to_degrees: 360.00 }
    }

    /// Gets the left direction.
    pub fn left() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 90.00, to_degrees: 270.00 }
    }

    /// Gets the right direction.
    pub fn right() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 270.00, to_degrees: 90.00 }
    }

    /// Gets the top left direction.
    pub fn top_left() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 90.00, to_degrees: 180.00 }
    }

    /// Gets the top right direction.
    pub fn top_right() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 0.00, to_degrees: 90.00 }
    }

    /// Gets the bottom left direction.
    pub fn bottom_left() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 180.00, to_degrees: 270.00 }
    }

    /// Gets the bottom right direction.
    pub fn bottom_right() -> EnemySpawnDirection {
        EnemySpawnDirection { from_degrees: 270.00, to_degrees: 360.00 }
    }
}


/// Spread of the enemy spawn.
#[derive(Debug, Default)]
pub struct EnemySpawnSpread {
    /// Minimum distance on the x-axis from the base position.
    pub x_min: f32,
    /// Maximum distance on the x-axis from the base position.
    pub x_max: f32,
    /// Minimum distance on the y-axis from the base position.
    pub y_min: f32,
    /// Maximum distance on the y-axis from the base position.
    pub y_max: f32,
}

impl EnemySpawnSpread {
    /// Creates a square spread.
    pub fn square(size: f32) -> EnemySpawnSpread {
        let spread = size / 2.00;
        EnemySpawnSpread { x_min: -spread, x_max: spread, y_min: -spread, y_max: spread }
    }
}
