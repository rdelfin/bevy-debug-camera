use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct ActiveGamepad(pub Option<Gamepad>);

/// This system signals whether the debug camera should be active. You can selectively pick which
/// input types are active at a given time. You can
#[derive(Resource, Debug)]
pub struct DebugCameraActive {
    /// If set to true, our keyboard + mouse bindings will be active for any cameras marked as
    /// [`crate::DebugCamera`].
    pub keymouse: bool,
    /// If set to true, our gamepad bindings will be active for any cameras marked as
    /// [`crate::DebugCamera`].
    pub gamepad: bool,
}

impl Default for DebugCameraActive {
    fn default() -> DebugCameraActive {
        DebugCameraActive {
            keymouse: true,
            gamepad: true,
        }
    }
}
