mod components;

pub use components::DebugCamera;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let camera = DebugCamera::default();
        assert_eq!(camera.speed_translate, 100.);
    }
}
