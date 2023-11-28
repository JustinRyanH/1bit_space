use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
struct MovementAttributes {
    #[export]
    speed: f64,

    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl IResource for MovementAttributes {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            speed: 30.0,
            base
        }
    }
}