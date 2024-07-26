use mythmallow_core_dependencies::*;

/// State of the diagnostics overlay.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum DiagnosticsOverlayState {
    /// Diagnostics overlay is disabled.
    #[default]
    Disabled,

    /// Diagnostics overlay is enabled.
    Enabled,
}
