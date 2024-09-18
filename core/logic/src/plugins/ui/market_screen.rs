use crate::{
    prelude::*,
    systems::ui::market_screen::*,
};

/// Plugin for managing the market screen of the game.
pub struct MarketScreenPlugin;

impl Plugin for MarketScreenPlugin {
    fn build(&self, app: &mut App) {
        // Register components.
        app.register_type::<MarketScreenBalanceText>();
        app.register_type::<MarketScreenRefreshButton>();
        app.register_type::<MarketScreenOfferingsContainer>();
        app.register_type::<MarketScreenBuyButton>();
        app.register_type::<MarketScreenLockButton>();

        // Add systems.
        app.add_systems(OnEnter(GameState::Market), spawn_market_screen);
        app.add_systems(
            PreUpdate,
            (
                update_balance_text.run_if(|balance: Res<Balance>| balance.is_changed()),
                update_refresh_button.run_if(
                    |balance: Res<Balance>,
                     market_configuration: Res<MarketConfiguration>,
                     market_state: Res<MarketState>| {
                        market_state.is_changed()
                            || market_configuration.is_changed()
                            || balance.is_changed()
                    },
                ),
            ),
        );

        app.add_systems(
            Last,
            update_offerings.run_if(
                |app_state: Res<State<AppState>>,
                 balance: Res<Balance>,
                 market_state: Res<MarketState>,
                 item_registry: Res<ItemRegistry>,
                 market_screen_offerings_container_query: Query<
                    Entity,
                    Added<MarketScreenOfferingsContainer>,
                >| {
                    app_state.get() == &AppState::Game
                        && (balance.is_changed()
                            || market_state.is_changed()
                            || item_registry.is_changed()
                            || !market_screen_offerings_container_query.is_empty())
                },
            ),
        );
    }
}
