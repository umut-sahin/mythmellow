//! Game over menu systems.

use {
    crate::utility::quit,
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_localizations::ui as localization,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};


/// Spawns the game over menu.
pub fn spawn_game_over_menu(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
    game_result: Res<GameResult>,
) {
    log::info!("spawning the game over menu");
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            match *game_result {
                GameResult::Won => {
                    column.menu_title(
                        localization::won_menu_title(),
                        &localization,
                        ui_font.clone(),
                    );
                    column
                        .menu_button(
                            localization::play_again_button(),
                            localization.deref(),
                            ui_font.clone(),
                        )
                        .named("Play Again Button")
                        .entity_commands()
                        .observe(on_play_again_button_clicked);
                },
                GameResult::Lost => {
                    column.menu_title(
                        localization::lost_menu_title(),
                        &localization,
                        ui_font.clone(),
                    );
                    column
                        .menu_button(
                            localization::retry_button(),
                            localization.deref(),
                            ui_font.clone(),
                        )
                        .named("Retry Button")
                        .entity_commands()
                        .observe(on_retry_button_clicked);
                },
            }

            // Spawn return to main menu button.
            column
                .menu_button(
                    localization::return_to_main_menu_button(),
                    localization.deref(),
                    ui_font.clone(),
                )
                .named("Return To Main Menu Button")
                .entity_commands()
                .observe(on_return_to_main_menu_button_clicked);

            // Spawn quit to desktop button.
            column
                .menu_button(
                    localization::quit_to_desktop_button(),
                    localization.deref(),
                    ui_font.clone(),
                )
                .named("Quit To Desktop Button")
                .entity_commands()
                .observe(on_quit_button_clicked);
        })
        .named("Game Over Menu")
        .insert(StateScoped(GameState::Over))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Restarts the game.
pub fn on_play_again_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("play again button is clicked");

    game_state_stack.transition(GameState::Restart);
    next_game_state.set(GameState::Transition);
}

/// Restarts the game.
pub fn on_retry_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("retry button is clicked");

    game_state_stack.transition(GameState::Restart);
    next_game_state.set(GameState::Transition);
}

/// Returns to the main menu.
pub fn on_return_to_main_menu_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("return to main menu button is clicked");
    log::info!("transitioning to the main menu");

    next_app_state.set(AppState::MainMenu);
}

/// Quits the application.
pub fn on_quit_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("quit button is clicked");
    log::info!("closing the application");

    quit(&mut app_exit_event_writer);
}
