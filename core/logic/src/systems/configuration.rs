//! Configuration systems.

use crate::prelude::*;


/// Selects the game mode, the player and the enemies then transitions to the game.
pub fn start_in_game(
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    arguments: Res<Arguments>,
    game_mode_registry: Res<GameModeRegistry>,
    player_registry: Res<PlayerRegistry>,
    enemy_registry: Res<EnemyRegistry>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_state_stack: ResMut<GameStateStack>,
) {
    log::info!("trying to start in game");

    {
        let mut game_mode_is_selected = false;
        match &arguments.start_in_game_mode {
            Some(specified_game_mode_id_and_args) => {
                let specified_game_mode_id =
                    specified_game_mode_id_and_args.split(' ').next().unwrap();
                if let Some(game_mode_index) = game_mode_registry
                    .iter()
                    .position(|game_mode| game_mode.id() == specified_game_mode_id)
                {
                    log::info!(
                        "selected manually specified {:?} mode as the game mode",
                        specified_game_mode_id,
                    );

                    commands.insert_resource(GameModeIndex(game_mode_index));

                    game_mode_is_selected = true;
                } else {
                    log::error!(
                        "couldn't select manually specified {:?} mode as the game mode \
                        as it doesn't exist",
                        specified_game_mode_id,
                    );
                }
            },
            None => {
                if !game_mode_registry.is_empty() {
                    let game_mode_index =
                        (0..game_mode_registry.len()).choose(rng.deref_mut()).unwrap();
                    let game_mode = &game_mode_registry[game_mode_index];

                    log::info!("randomly selected {:?} mode as the game mode", game_mode.id());

                    commands.insert_resource(GameModeIndex(game_mode_index));

                    game_mode_is_selected = true;
                } else {
                    log::error!(
                        "couldn't select the game mode randomly as no game modes are registered",
                    );
                }
            },
        }
        if !game_mode_is_selected {
            log::info!("transitioning to the game mode selection screen");
            next_app_state.set(AppState::GameModeSelectionScreen);
            return;
        }
    }
    {
        let mut player_is_selected = false;
        match &arguments.start_in_game_player {
            Some(specified_player_id_and_args) => {
                let specified_player_id = specified_player_id_and_args.split(' ').next().unwrap();
                match player_registry.find_player(specified_player_id) {
                    Some((mythology_index, player_index)) => {
                        log::info!(
                            "selected manually specified {:?} from {:?} mythology as the player",
                            specified_player_id,
                            player_registry[mythology_index].id(),
                        );

                        commands.insert_resource(MythologyIndex(mythology_index));
                        commands.insert_resource(PlayerIndex(player_index));

                        player_is_selected = true;
                    },
                    None => {
                        log::error!(
                            "couldn't select manually specified {:?} as the player \
                            as it doesn't exist",
                            specified_player_id,
                        );
                    },
                }
            },
            None => {
                if !player_registry.is_empty() {
                    let mythology_index =
                        (0..player_registry.len()).choose(rng.deref_mut()).unwrap();
                    let mythology = &player_registry[mythology_index];

                    let player_index =
                        (0..mythology.players.len()).choose(rng.deref_mut()).unwrap();
                    let player = &mythology.players[player_index];

                    log::info!(
                        "randomly selected {:?} from {:?} mythology as the player",
                        player.id(),
                        mythology.id(),
                    );

                    commands.insert_resource(MythologyIndex(mythology_index));
                    commands.insert_resource(PlayerIndex(player_index));

                    player_is_selected = true;
                } else {
                    log::error!("couldn't select the player randomly as no players are registered");
                }
            },
        }
        if !player_is_selected {
            log::info!("transitioning to the player selection screen");
            next_app_state.set(AppState::PlayerSelectionScreen);
            return;
        }
    }
    {
        let mut enemy_pack_is_selected = false;
        match &arguments.start_in_game_enemies {
            Some(specified_enemy_pack_id_and_args) => {
                let specified_enemy_pack_id =
                    specified_enemy_pack_id_and_args.split(' ').next().unwrap();
                if let Some(enemy_pack_index) = enemy_registry
                    .iter()
                    .position(|enemy_pack| enemy_pack.id() == specified_enemy_pack_id)
                {
                    log::info!(
                        "selected manually specified {:?} enemies as the opponents",
                        specified_enemy_pack_id,
                    );

                    commands.insert_resource(EnemyPackIndex(enemy_pack_index));

                    enemy_pack_is_selected = true;
                } else {
                    log::error!(
                        "couldn't select manually specified {:?} enemies as the opponents \
                        as they don't exist",
                        specified_enemy_pack_id,
                    );
                }
            },
            None => {
                if !player_registry.is_empty() {
                    let enemy_pack_index =
                        (0..enemy_registry.len()).choose(rng.deref_mut()).unwrap();
                    let enemy_pack = &enemy_registry[enemy_pack_index];

                    log::info!("randomly selected {:?} enemies as the opponents", enemy_pack.id());

                    commands.insert_resource(EnemyPackIndex(enemy_pack_index));

                    enemy_pack_is_selected = true;
                } else {
                    log::error!(
                        "couldn't select the opponents randomly as no enemies are registered",
                    );
                }
            },
        }
        if !enemy_pack_is_selected {
            log::info!("transitioning to the enemy selection screen");
            next_app_state.set(AppState::EnemySelectionScreen);
            return;
        }
    }

    log::info!("transitioning to the game");

    game_state_stack.push(GameState::Initialization);
    next_app_state.set(AppState::Game);
}
