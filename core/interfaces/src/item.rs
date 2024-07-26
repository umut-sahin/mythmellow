use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
};

/// Interface for the items.
pub trait IItem: Debug + Send + Sync + 'static {
    /// Gets the unique identifier of the item.
    fn id(&self) -> SmolStr;

    /// Gets the localized name of the item.
    fn name(&self) -> LocalizedText;


    /// Gets whether the item is a weapon.
    fn is_weapon(&self) -> bool;

    /// Gets the base range of the weapon.
    fn base_range(&self) -> Option<Range> {
        if self.is_weapon() {
            panic!("weapons need to provide a base range");
        } else {
            None
        }
    }


    /// Instantiates the item to add it to the inventory.
    fn instantiate(&self) -> ItemInstance;

    /// Acquires the item.
    fn acquire(&self, world: &mut World) -> Entity;

    /// Releases the item.
    fn release(&self, world: &mut World, entity: Entity);
}


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
