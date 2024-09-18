use crate::{
    prelude::*,
    systems::market::*,
};

/// Plugin for managing the market of the game.
pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<Balance>();
        app.register_type::<ExperienceToBalanceRatio>();
        app.register_type::<MarketConfiguration>();
        app.register_type::<MarketState>();

        // Insert resources.
        app.insert_resource(Balance::default());
        app.insert_resource(ExperienceToBalanceRatio::default());
        app.insert_resource(MarketConfiguration::default());
        app.insert_resource(MarketState::default());

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            (
                initialize_market.in_set(InitializationSystems::Market),
                configure_market.in_set(InitializationSystems::Last),
            ),
        );
        app.add_systems(
            PreUpdate,
            (
                gain_balance.run_if(resource_exists::<Balance>),
                process_acquirements.run_if(resource_changed::<MarketState>),
            ),
        );
        app.add_systems(
            Update,
            refresh_market_automatically.run_if(
                |game_state: Option<Res<State<GameState>>>,
                 market_configuration: Res<MarketConfiguration>,
                 market_state: Res<MarketState>| {
                    let Some(game_state) = game_state else { return false };

                    match game_state.get() {
                        GameState::Market => {},
                        GameState::Playing | GameState::Paused => {
                            if market_state.offered_item_ids.is_empty() {
                                return false;
                            }
                        },
                        _ => return false,
                    }

                    market_state.offered_item_ids.len()
                        != (market_configuration.number_of_items as usize)
                },
            ),
        );
        app.add_systems(OnExit(InGame), deinitialize_market);
    }
}
