//! State systems.

use {
    mythmallow_core_actions::GameAction,
    mythmallow_core_components::all::Player,
    mythmallow_core_dependencies::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};

/// Transitions to the loading state.
pub fn start_loading(
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game_state_stack.transition(GameState::Loading);
    next_game_state.set(GameState::Transition);
}

/// Transitions to the playing state.
pub fn start_playing(
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game_state_stack.transition(GameState::Playing);
    next_game_state.set(GameState::Transition);
}

/// Transitions to the initialization state.
pub fn restart(
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    game_state_stack.transition(GameState::Initialization);
    next_game_state.set(GameState::Transition);
}


/// Transitions to the state at the top of the game state stack.
pub fn game_state_transition(
    mut game_action_state_query: Query<&mut ActionState<GameAction>, With<Player>>,
    game_state_stack: Res<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Some(game_state) = game_state_stack.last() {
        log::info!("{}", game_state);
        next_game_state.set(*game_state);

        if game_state == &GameState::Playing {
            if let Ok(mut game_action_state) = game_action_state_query.get_single_mut() {
                game_action_state.enable();
            }
        }
    }
}
