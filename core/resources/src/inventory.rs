//! Inventory resources.

use {
    mythmallow_core_dependencies::*,
    mythmallow_core_interfaces::*,
};


/// Resource for the inventory of the player.
#[derive(Debug, Default, Resource)]
pub struct Inventory {
    /// Items in the inventory.
    pub items: Vec<Arc<ItemInstance>>,

    /// Items to add to the inventory.
    pub items_to_add: Vec<ItemInstance>,

    /// Items to remove from the inventory.
    pub items_to_remove: Vec<Arc<ItemInstance>>,
}

impl Inventory {
    /// Adds an item to the inventory.
    pub fn add(&mut self, item: ItemInstance) {
        self.items_to_add.push(item);
    }

    /// Removes an item from the inventory.
    pub fn remove(&mut self, item: Arc<ItemInstance>) {
        self.items_to_remove.push(item);
    }
}

impl Deref for Inventory {
    type Target = Vec<Arc<ItemInstance>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
