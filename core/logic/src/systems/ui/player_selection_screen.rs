//! Player selection screen systems.

use {
    crate::{
        prelude::*,
        systems::utility::*,
    },
    localizations::ui as localization,
};


/// Spawns the player selection screen.
pub fn spawn_player_selection_screen(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
    player_registry: Res<PlayerRegistry>,
) {
    log::info!("spawning the player selection screen");
    commands
        .ui_builder(UiRoot)
        .column(|root| {
            if player_registry.is_empty() {
                // Spawn the no players title.
                root.menu_title(
                    localization::no_players_menu_title(),
                    &localization,
                    ui_font.clone(),
                );
            } else {
                // Spawn the player buttons.
                for (mythology_index, entry) in player_registry.iter().enumerate() {
                    for (player_index, player) in entry.players.iter().enumerate() {
                        let mut player_button =
                            root.menu_button(player.name(), localization.deref(), ui_font.clone());
                        player_button.named(format!("Player Button [{}]", player.id())).insert(
                            PlayerSelectionScreenPlayerButton { mythology_index, player_index },
                        );
                        if mythology_index == 0 && player_index == 0 {
                            player_button.insert(WidgetSelected::now());
                        }
                        player_button.entity_commands().observe(on_player_button_clicked);
                    }
                }
            }

            // Spawn the back button.
            {
                let mut back_button = root.menu_button(
                    localization::back_button(),
                    localization.deref(),
                    ui_font.clone(),
                );
                back_button.named("Back Button");
                if player_registry.is_empty() {
                    back_button.insert(WidgetSelected::now());
                }
                back_button.entity_commands().observe(on_back_button_clicked);
            }
        })
        .named("Player Selection Screen")
        .insert(StateScoped(AppState::PlayerSelectionScreen))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Transitions to the enemy selection screen.
pub fn on_player_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    player_button_query: Query<&PlayerSelectionScreenPlayerButton>,
    player_registry: Res<PlayerRegistry>,
    mut next_app_state: ResMut<NextState<AppState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    let button = match player_button_query.get(trigger.entity()) {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    log::info!(
        "{:?} player button is clicked",
        player_registry[button.mythology_index].players[button.player_index].id(),
    );
    log::info!("transitioning to the enemy selection screen");

    commands.insert_resource(MythologyIndex(button.mythology_index));
    commands.insert_resource(PlayerIndex(button.player_index));

    commands.run_system(registered_systems.widget.save_selected_widget);
    next_app_state.set(AppState::EnemySelectionScreen);
}

/// Transitions to the game mode selection screen.
pub fn on_back_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("back button is clicked");
    log::info!("transitioning to the game mode selection screen");

    commands.insert_resource(RestorePreviouslySelectedWidget);
    next_app_state.set(AppState::GameModeSelectionScreen);
}
