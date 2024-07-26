use {
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
};


/// Systems to run in [GameState::Playing].
#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq, SystemSet)]
pub enum GameplaySystems {
    /// Game mode systems.
    GameMode,

    /// Movement systems.
    Movement,

    /// Camera systems.
    Camera,

    /// Player systems.
    Player,

    /// Enemy systems.
    Enemy,

    /// Item systems.
    Item,

    /// Combat systems.
    Combat,
}

impl GameplaySystems {
    /// Configures the system set.
    pub fn configure(app: &mut App) {
        fn run_condition(
            game_state: Option<Res<State<GameState>>>,
            console_state: Res<ConsoleState>,
        ) -> bool {
            matches!(game_state, Some(game_state) if game_state.deref() == &GameState::Playing)
                && !console_state.open
        }

        for set in GameplaySystems::iter() {
            app.configure_sets(FixedUpdate, set.run_if(run_condition));
            app.configure_sets(PreUpdate, set.run_if(run_condition));
            app.configure_sets(Update, set.run_if(run_condition));
            app.configure_sets(PostUpdate, set.run_if(run_condition));
        }
    }
}
