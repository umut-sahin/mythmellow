//! Item resources.

use crate::prelude::*;


/// Container for the items in the inventory.
#[derive(Debug)]
pub struct ItemInstance {
    /// Item.
    pub item: Box<dyn IItem>,
    /// Entity of the item.
    pub entity: Option<Entity>,
}

impl ItemInstance {
    /// Creates a new item instance.
    pub fn new(item: impl IItem) -> ItemInstance {
        ItemInstance { item: Box::new(item), entity: None }
    }
}

impl Deref for ItemInstance {
    type Target = Box<dyn IItem>;

    fn deref(&self) -> &Box<dyn IItem> {
        &self.item
    }
}


/// Resource for the available balance to spend in the market.
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, PartialOrd, PartialEq, Reflect, Resource)]
pub struct Balance(pub f64);

impl Balance {
    /// Zero balance.
    pub const ZERO: Balance = Balance(0.00);
}

impl Balance {
    /// Sets the balance.
    pub fn set(&mut self, amount: Balance) {
        *self = amount;
        log::info!("setting the balance to {}", self);
    }

    /// Records the gains.
    pub fn gain(&mut self, amount: Balance, reason: impl AsRef<str>) {
        log::info!("gaining {} from {}", amount, reason.as_ref());
        self.0 += amount.0;
        log::info!("new balance is {}", self);
    }

    /// Records the losses.
    pub fn spend(&mut self, amount: Balance, reason: impl AsRef<str>) {
        log::info!("spending {} to {}", amount, reason.as_ref());
        self.0 -= amount.0;
        log::info!("new balance is {}", self);
    }
}

impl Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_nan() {
            return write!(f, "?");
        }
        if self.is_infinite() {
            write!(f, "{}∞", if self.is_sign_positive() { "" } else { "-" })
        } else {
            write!(f, "{:.2} $", self.0)
        }
    }
}
