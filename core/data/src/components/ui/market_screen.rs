//! Market screen components.

use crate::prelude::*;


/// Component for the balance text in the market screen.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct MarketScreenBalanceText;


/// Component for the refresh button in the market screen.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct MarketScreenRefreshButton {
    /// Cost of using the button.
    pub cost: Balance,
}


/// Component for the label of the refresh button in the market screen.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct MarketScreenRefreshButtonLabel;


/// Component for the offerings container in the market screen.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct MarketScreenOfferingsContainer;


/// Component for the offering containers in the market screen.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct MarketScreenOfferingContainer;


/// Component for the buy buttons in the market screen.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct MarketScreenBuyButton {
    /// Index of the item.
    pub item_index: usize,
    /// Price of the item.
    pub price: Balance,
}


/// Component for the lock buttons in the market screen.
#[derive(Clone, Component, Copy, Debug, Default, Reflect)]
pub struct MarketScreenLockButton {
    /// Index of the item.
    pub item_index: usize,
}
