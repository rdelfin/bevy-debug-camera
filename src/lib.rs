mod components;
mod resources;
mod systems;

pub use components::DebugCamera;
pub use resources::{ActiveGamepad, DebugCameraActive};

use bevy::prelude::*;

pub struct DebugCameraPlugin;

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::ActiveGamepad::default())
            .insert_resource(resources::DebugCameraActive::default())
            .add_system(systems::camera_movement_system)
            .add_system(systems::camera_update_system)
            .add_system(systems::cursor_grab_system)
            .add_system(systems::gamepad_connections);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let camera = DebugCamera::default();
        assert_eq!(camera.speed_translate, 100.);
    }
}
