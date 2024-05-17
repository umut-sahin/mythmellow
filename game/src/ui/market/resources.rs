use crate::prelude::*;


/// Resource for the widgets of the market.
#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct MarketWidgets(
    /// Rows of widgets in the market.
    ///
    /// - Balance
    /// - Buy buttons
    /// - Lock buttons
    /// - Refresh button
    pub [Vec<Entity>; 4],
);