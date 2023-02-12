# Bevy Debug Camera

This library provides a simple, lightweight way of having a "flycam" style camera for
debugging. It's confirgurable, letting you easily enable the system locally for a camera or
globally for all cameras with a component and resource.

# Examples

You can look at the examples folder for
practical uses of this crate, but to get started you can simply do the following when setting
up your app:

```
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

```
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
