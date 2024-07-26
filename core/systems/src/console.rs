//! Console systems.

use {
    mythmallow_core_dependencies::*,
    mythmallow_core_states::*,
};


/// Sets the console size based on the size of the primary window.
pub fn set_console_size(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut console_configuration: ResMut<ConsoleConfiguration>,
) {
    if let Ok(primary_window) = primary_window_query.get_single() {
        let window_width = primary_window.width();
        let left_margin = window_width * 0.10;
        let console_width = window_width * 0.80;

        let window_height = primary_window.height();
        let top_margin = window_height * 0.10;
        let console_height = window_height * 0.80;

        console_configuration.left_pos = left_margin;
        console_configuration.width = console_width;
        console_configuration.top_pos = top_margin;
        console_configuration.height = console_height;
    }
}


/// (Un)pauses physics time depending on the console state.
pub fn control_physics_time(
    mut physics_time: ResMut<Time<Physics>>,
    console_state: Res<ConsoleState>,
    game_state: Option<Res<State<GameState>>>,
) {
    if console_state.open {
        physics_time.pause();
    } else if let Some(game_state) = game_state {
        if game_state.get() == &GameState::Playing {
            physics_time.unpause();
        }
    }
}
