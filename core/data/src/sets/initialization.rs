use crate::prelude::*;

/// Systems to run in [GameState::Initialization].
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum InitializationSystems {
    /// Systems that run before everything else.
    First,

    /// Market systems.
    Market,

    /// Head-up display systems.
    Hud,

    /// Game mode systems.
    GameMode,

    /// Player systems.
    Player,

    /// Leveling systems.
    Leveling,

    /// Systems that run after everything else.
    Last,

    /// Systems that run when initialization is done.
    Done,
}

impl InitializationSystems {
    /// Configures the system set.
    pub fn configure(app: &mut App) {
        for (current, next) in
            InitializationSystems::iter().zip(InitializationSystems::iter().skip(1))
        {
            app.configure_sets(OnEnter(GameState::Initialization), current.before(next));
            app.configure_sets(OnExit(GameState::Initialization), current.before(next));
        }
    }
}
