# Bevy Debug Camera

This library provides a simple, lightweight way of having a "flycam" style camera for
debugging. It's confirgurable, letting you easily enable the system locally for a camera or
globally for all cameras with a component and resource.

# Bevy compatibility

| Bevy Version | bevy-debug-camera version |
|--------------|---------------------------|
| 0.9.1        | ^0.1.0                    |

# Examples

You can look at the examples folder for
practical uses of this crate, but to get started you can simply do the following when setting
up your app:

```rust
use bevy::prelude::*;
use bevy_debug_camera::{DebugCamera, DebugCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugCameraPlugin)
        .add_startup_system(setup)
        .run();
}
```

And, when setting up your camera you can do the following:

```rust
fn setup(mut commands: Commands) {
    // ... other setup code
    commands
        .spawn(Camera3dBundle::default())
        .insert(DebugCamera {
            position: Vec3::new(-5., 1., 0.),
            ..default(),
        });
}
```

# Bindings

The default bindings are as follows:

## Mouse + Keyboard

| Action        | Binding  |
|---------------|----------|
| Move forward  | `W`      |
| Move backward | `S`      |
| Move left     | `A`      |
| Move right    | `D`      |
| Move up       | `Lshift` |
| Move down     | `Space`  |
| Yaw           | Mouse X  |
| pitch         | Mouse Y  |
| Roll left     | `Q`      |
| Roll right    | `E`      |

## Controller

| Action          | Binding    |
|-----------------|------------|
| Move fwd/bwd    | Lstick Y   |
| Move left/right | Lstick X   |
| Move up         | `RTrigger` |
| Move down       | `LTrigger` |
| Yaw             | Rstick X   |
| pitch           | Lstick Y   |
| Roll left       | `LBumper`  |
| Roll right      | `RBumper`  |

# Configuring Plugin

The plugin comes with some configuration options you can set on startup that use to customise
behaviour of the cameras in use. You can configure:

* Keyboard bindings
* Gamepad bindings
* Accepted input

All these customisation are exposed as resources, which are constantly read and can be modified
during runtime as well An example using all configuration options can be seen below and in the
`configuration` example:

```rust
use bevy::prelude::*;
use bevy_debug_camera::{
    DebugCamera, DebugCameraPlugin, GamepadBindings, KeyboardBindings, DebugCameraActive,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Each field in `DebugCameraPlugin` can be set directly or picked up from
        // default.
        .add_plugin(DebugCameraPlugin {
            gamepad_bindings: GamepadBindings {
                // Overrides only the roll buttons
                roll_left: GamepadButtonType::West,
                roll_right: GamepadButtonType::East,
                ..default()
            },
            keyboard_bindings: KeyboardBindings {
                // Override WASD with arrows
                fwd: KeyCode::Up,
                bwd: KeyCode::Down,
                left: KeyCode::Left,
                right: KeyCode::Right,
                ..default()
            },
            debug_camera_active: DebugCameraActive {
                // Disable keyboard + mouse only
                keymouse: false,
                ..default()
            },
        })
        .add_startup_system(setup)
        .run();
}
```
