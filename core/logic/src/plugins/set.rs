use crate::prelude::*;

/// Plugin for managing the system sets of the application.
pub struct SetPlugin;

impl Plugin for SetPlugin {
    fn build(&self, app: &mut App) {
        // Configure system sets.
        InitializationSystems::configure(app);
        LoadingSystems::configure(app);
        GameplaySystems::configure(app);
        RestartSystems::configure(app);
    }
}
