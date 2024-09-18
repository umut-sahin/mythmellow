use crate::{
    prelude::*,
    systems::action::*,
};

/// Plugin for managing the actions of the application.
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        // Setup actions.
        GlobalAction::setup(app);
        MenuAction::setup(app);
        GameAction::setup(app);

        // Add systems.
        {
            app.add_systems(
                Update,
                (
                    pause_on_losing_focus.in_set(GameplaySystems::Action),
                    toggle_fullscreen,
                    toggle_diagnostics_overlay,
                ),
            );
            #[cfg(feature = "development")]
            app.add_systems(Update, toggle_physics_gizmos);

            app.add_systems(
                PostUpdate,
                (
                    pause.in_set(GameplaySystems::Player),
                    open_market.in_set(GameplaySystems::Market),
                    close_market.run_if(
                        |game_state: Option<Res<State<GameState>>>,
                         console_state: Res<ConsoleState>| {
                            matches!(
                                game_state,
                                Some(game_state) if game_state.get() == &GameState::Market,
                            ) && !console_state.open
                        },
                    ),
                ),
            );
        }
    }
}
