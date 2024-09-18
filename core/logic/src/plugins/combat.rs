use crate::{
    prelude::*,
    systems::{
        combat::*,
        utility::*,
    },
};

/// Plugin for managing the combat of the game.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<Attack>();
        app.register_type::<DamagePlayerOnContact>();
        app.register_type::<DamagePlayerOnContactStarted>();
        app.register_type::<DamageEnemiesOnContact>();
        app.register_type::<DamageEnemiesOnContactStarted>();
        app.register_type::<DamageCooldown>();
        app.register_type::<Originator>();
        app.register_type::<Projectile>();

        // Add systems.
        {
            app.add_systems(
                PreUpdate,
                (cooldown::<Attack>, cooldown::<Damage>).in_set(GameplaySystems::Combat),
            );

            app.add_systems(PostUpdate, start_attack_animations.in_set(GameplaySystems::Combat));
            app.add_systems(OnExit(GameState::Playing), pause_attack_animations);
            app.add_systems(
                Last,
                pause_attack_animations.run_if(|console_state: Res<ConsoleState>| {
                    console_state.is_changed() && console_state.open
                }),
            );
            app.add_systems(
                OnEnter(GameState::Playing),
                resume_attack_animations
                    .run_if(|console_state: Res<ConsoleState>| !console_state.open),
            );
            app.add_systems(
                Last,
                resume_attack_animations.run_if(
                    |game_state: Option<Res<State<GameState>>>,
                     console_state: Res<ConsoleState>| {
                        console_state.is_changed()
                            && !console_state.open
                            && matches!(
                                game_state,
                                Some(game_state) if game_state.get() == &GameState::Playing,
                            )
                    },
                ),
            );

            app.add_systems(
                Update,
                (
                    damage_player_on_contact,
                    damage_player_on_contact_started,
                    damage_enemies_on_contact,
                    damage_enemies_on_contact_started,
                )
                    .in_set(GameplaySystems::Combat),
            );
            app.add_systems(
                PostUpdate,
                (
                    player_death.run_if(god_mode_is_disabled),
                    enemy_death,
                    despawn_projectiles_on_contact,
                )
                    .in_set(GameplaySystems::Combat),
            );
        }
    }
}
