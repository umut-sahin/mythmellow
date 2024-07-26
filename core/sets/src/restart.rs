use {
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
};


/// Systems to run in [GameState::Restart].
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum RestartSystems {
    /// Systems that run before everything else.
    First,

    /// Game mode systems.
    GameMode,

    /// Systems that run after everything else.
    Last,

    /// Systems that run when restarting is done.
    Done,
}

impl RestartSystems {
    /// Configures the system set.
    pub fn configure(app: &mut App) {
        for (current, next) in RestartSystems::iter().zip(RestartSystems::iter().skip(1)) {
            app.configure_sets(OnEnter(GameState::Restart), current.before(next));
            app.configure_sets(OnExit(GameState::Restart), current.before(next));
        }
    }
}
