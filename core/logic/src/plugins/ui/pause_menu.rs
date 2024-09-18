use crate::{
    prelude::*,
    systems::ui::pause_menu::*,
};

/// Plugin for managing the pause menu of the application.
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        // Add systems.
        app.add_systems(OnEnter(GameState::Paused), spawn_pause_menu);
    }
}
