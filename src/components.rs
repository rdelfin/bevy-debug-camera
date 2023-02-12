use bevy::prelude::*;

/// Any entity with this component will be controllable using the default bindings for
/// this plugin. For more information on controls, refer to the crate root.
#[derive(Debug, Component)]
pub struct DebugCamera {
    /// Up vector for the camera. Will be kept as a unit vector and perpendicular to fwd
    /// automatically by our systems. Should not be initialised to be co-linear with fwd.
    pub up: Vec3,
    /// Forward vector for the camera. Will be kept as a unit vector and perpendicular to up
    /// automatically by out systems. Should not be initialised to be co-linear with up.
    pub fwd: Vec3,
    /// The position in global space of the camera. Should be initialised by the user. We will
    /// update this automatically.
    pub position: Vec3,
    /// This is a configurable setting for this camera. It is the speed (in units/second) at which
    /// the camera should translate when going at full speed.
    pub speed_translate: f32,
    /// This is a configurable setting for this camera. It is the speed (in radians/second) at
    /// which the camera should rotate when going at full speed.
    pub speed_rotate: f32,
}

impl Default for DebugCamera {
    fn default() -> DebugCamera {
        DebugCamera {
            up: Vec3::new(0., 1., 0.),
            fwd: Vec3::new(1., 0., 0.),
            position: Vec3::default(),
            speed_translate: 100.,
            speed_rotate: std::f32::consts::FRAC_PI_4,
        }
    }
}
