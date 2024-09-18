use crate::prelude::*;

/// Actions that can be performed in the menus.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum MenuAction {
    /// Select the widget above the currently selected widget.
    Up,

    /// Select the widget to the left of the currently selected widget.
    Left,

    /// Select the widget below the currently selected widget.
    Down,

    /// Select the widget to the right of the currently selected widget.
    Right,

    /// Click the currently selected widget.
    Click,

    /// Go back to the previous state.
    Back,
}

impl MenuAction {
    /// Sets up the action.
    pub fn setup(app: &mut App) {
        // Add input manager plugin.
        app.add_plugins(InputManagerPlugin::<MenuAction>::default());

        // Create the input map.
        let mut input_map = InputMap::new([
            (MenuAction::Click, KeyCode::Enter),
            (MenuAction::Back, KeyCode::Escape),
        ]);

        // Extend the input map from key bindings.
        let key_bindings = app.world_mut().resource::<Persistent<KeyBindings>>();
        for key_code in key_bindings.up.iter().cloned() {
            input_map.insert(MenuAction::Up, key_code);
        }
        for key_code in key_bindings.left.iter().cloned() {
            input_map.insert(MenuAction::Left, key_code);
        }
        for key_code in key_bindings.down.iter().cloned() {
            input_map.insert(MenuAction::Down, key_code);
        }
        for key_code in key_bindings.right.iter().cloned() {
            input_map.insert(MenuAction::Right, key_code);
        }

        // Insert the input map resource.
        app.insert_resource(input_map);

        // Insert the global action state as a resource.
        app.insert_resource(ActionState::<MenuAction>::default());
    }
}
