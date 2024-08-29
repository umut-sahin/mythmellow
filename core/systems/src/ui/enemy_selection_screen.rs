//! Enemy selection screen systems.

use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_localizations::ui as localization,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_states::*,
};


/// Spawns the enemy selection screen.
pub fn spawn_enemy_selection_screen(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
    enemy_registry: Res<EnemyRegistry>,
) {
    log::info!("spawning the enemy selection screen");
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            if enemy_registry.is_empty() {
                // Spawn no enemies title.
                column.menu_title(
                    localization::no_enemies_menu_title(),
                    &localization,
                    ui_font.clone(),
                );
            } else {
                // Spawn enemy buttons.
                for (enemy_pack_index, enemy_pack) in enemy_registry.iter().enumerate() {
                    let mut enemy_button = column.menu_button(
                        enemy_pack.name(),
                        localization.deref(),
                        ui_font.clone(),
                    );
                    enemy_button
                        .named(format!("Enemy Button [{}]", enemy_pack.id()))
                        .insert(EnemySelectionScreenEnemyButton { enemy_pack_index });
                    if enemy_pack_index == 0 {
                        enemy_button.insert(WidgetSelected::now());
                    }
                    enemy_button.entity_commands().observe(on_enemy_button_clicked);
                }
            }

            // Spawn back button.
            {
                let mut back_button = column.menu_button(
                    localization::back_button(),
                    localization.deref(),
                    ui_font.clone(),
                );
                back_button.named("Back Button");
                if enemy_registry.is_empty() {
                    back_button.insert(WidgetSelected::now());
                }
                back_button.entity_commands().observe(on_back_button_clicked);
            }
        })
        .named("Enemy Selection Screen")
        .insert(StateScoped(AppState::EnemySelectionScreen))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(1.50));
}


/// Transitions to the game.
pub fn on_enemy_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    enemy_button_query: Query<&EnemySelectionScreenEnemyButton>,
    enemy_registry: Res<EnemyRegistry>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    let button = match enemy_button_query.get(trigger.entity()) {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    log::info!("{:?} enemies button is clicked", enemy_registry[button.enemy_pack_index].id());
    log::info!("transitioning to the game");

    commands.insert_resource(EnemyPackIndex(button.enemy_pack_index));

    game_state_stack.push(GameState::Initialization);
    next_app_state.set(AppState::Game);

    // No need to remember the previously selected widgets now.
    // As the game will start and there won't be a way to go back.
    commands.init_resource::<PreviouslySelectedWidgetStack>();
}

/// Transitions to the player selection screen.
pub fn on_back_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("back button is clicked");
    log::info!("transitioning to the player selection screen");

    commands.insert_resource(RestorePreviouslySelectedWidget);
    next_app_state.set(AppState::PlayerSelectionScreen);
}
