//! Widget resources.

use mythmallow_core_dependencies::*;


/// Resource for the font of the user interface.
#[derive(Deref, Reflect, Resource)]
pub struct UiFont(pub Handle<Font>);


/// Container for the neighboring widgets.
#[derive(Clone, Debug, Reflect)]
pub struct NeighboringWidgets {
    /// Widget above.
    pub up: Entity,
    /// Widget to left.
    pub left: Entity,
    /// Widget below.
    pub down: Entity,
    /// Widget to right.
    pub right: Entity,
}


/// Resource for the widget structure.
#[derive(Debug, Default, Deref, DerefMut, Reflect, Resource)]
pub struct WidgetStructure(pub Vec<(Entity, NeighboringWidgets)>);

impl WidgetStructure {
    /// Gets the index of a widget if it's in the widget structure.
    pub fn index_of(&self, entity: Entity) -> Option<usize> {
        self.0.iter().position(|(candidate, _)| *candidate == entity)
    }

    /// Gets the neighbors of a widget if it's in the widget structure.
    pub fn neighbors_of(&self, entity: Entity) -> Option<&NeighboringWidgets> {
        self.0.iter().find(|(candidate, _)| *candidate == entity).map(|(_, neighbors)| neighbors)
    }
}


/// Resource for the indices of the previously selected widgets.
#[derive(Debug, Default, Deref, DerefMut, Reflect, Resource)]
pub struct PreviouslySelectedWidgetStack(pub Vec<usize>);


/// Resource for the flag to restore the previously selected widget.
#[derive(Debug, Default, Reflect, Resource)]
pub struct RestorePreviouslySelectedWidget;


/// Resource to indicate whether the previously selected widget is restored.
#[derive(Debug, Default, Reflect, Resource)]
pub struct PreviouslySelectedWidgetRestored;
