use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct ActiveGamepad(pub Option<Gamepad>);

/// This system signals whether the debug camera should be active. You can selectively pick which
/// input types are active at a given time. You can
#[derive(Resource, Debug, Clone)]
pub struct DebugCameraActive {
    /// If set to true, our keyboard + mouse bindings will be active for any cameras marked as
    /// [`crate::DebugCamera`].
    pub keymouse: bool,
    /// If set to true, our gamepad bindings will be active for any cameras marked as
    /// [`crate::DebugCamera`].
    pub gamepad: bool,
    // if true, then debug camera is frozen in place until next esc key is pressed
    pub(crate) esc_toggled: bool,
}

impl Default for DebugCameraActive {
    fn default() -> DebugCameraActive {
        DebugCameraActive {
            keymouse: true,
            gamepad: true,
            esc_toggled: false,
        }
    }
}

/// Configurable bindings for keyboard input. Field defaults can be found in the crate root
/// documentation.
#[derive(Resource, Debug, Clone)]
pub struct KeyboardBindings {
    pub esc: KeyCode,
    pub fwd: KeyCode,
    pub bwd: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub roll_left: KeyCode,
    pub roll_right: KeyCode,
}

impl Default for KeyboardBindings {
    fn default() -> KeyboardBindings {
        KeyboardBindings {
            esc: KeyCode::Escape,
            fwd: KeyCode::W,
            bwd: KeyCode::S,
            up: KeyCode::Space,
            down: KeyCode::ShiftLeft,
            left: KeyCode::A,
            right: KeyCode::D,
            roll_left: KeyCode::Q,
            roll_right: KeyCode::E,
        }
    }
}

/// Configurable bindings for gamepad input. Field defaults can be found in the crate root
/// documentation.
#[derive(Resource, Debug, Clone)]
pub struct GamepadBindings {
    pub esc: GamepadButtonType,
    pub fwd_bwd: GamepadAxisType,
    pub up: GamepadButtonType,
    pub down: GamepadButtonType,
    pub left_right: GamepadAxisType,
    pub roll_left: GamepadButtonType,
    pub roll_right: GamepadButtonType,
    pub yaw: GamepadAxisType,
    pub pitch: GamepadAxisType,
}

impl Default for GamepadBindings {
    fn default() -> GamepadBindings {
        GamepadBindings {
            esc: GamepadButtonType::Mode,
            fwd_bwd: GamepadAxisType::LeftStickY,
            up: GamepadButtonType::RightTrigger2,
            down: GamepadButtonType::LeftTrigger2,
            left_right: GamepadAxisType::LeftStickX,
            roll_left: GamepadButtonType::LeftTrigger,
            roll_right: GamepadButtonType::RightTrigger,
            yaw: GamepadAxisType::RightStickX,
            pitch: GamepadAxisType::RightStickY,
        }
    }
}
