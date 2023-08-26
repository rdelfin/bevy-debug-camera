//! An example demonstrating how you might configure the different settings provided by the
//! [`DebugCameraPlugin`]. Feel free to play around with settings and see what happens.

use bevy::prelude::*;
use bevy_debug_camera::{
    DebugCamera, DebugCameraActive, DebugCameraPlugin, GamepadBindings, KeyboardBindings,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Each field in `DebugCameraPlugin` can be set directly or picked up from
        // default.
        .add_plugins(DebugCameraPlugin {
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
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            ..default()
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(DebugCamera {
            position: Vec3::new(-5., 2., 0.),
            up: Vec3::new(0., 1., 0.),
            fwd: Vec3::new(1., 0., 0.),
            ..default()
        });
}
