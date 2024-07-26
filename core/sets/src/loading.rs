use {
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
};


/// Systems to run in [GameState::Loading].
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum LoadingSystems {
    /// Systems that run before everything else.
    First,

    /// Game mode systems.
    GameMode,

    /// Camera systems.
    Camera,

    /// Enemy systems.
    Enemy,

    /// Systems that run after everything else.
    Last,

    /// Systems that run when loading is done.
    Done,
}

impl LoadingSystems {
    /// Configures the system set.
    pub fn configure(app: &mut App) {
        for (current, next) in LoadingSystems::iter().zip(LoadingSystems::iter().skip(1)) {
            app.configure_sets(OnEnter(GameState::Loading), current.before(next));
            app.configure_sets(OnExit(GameState::Loading), current.before(next));
        }
    }
}
