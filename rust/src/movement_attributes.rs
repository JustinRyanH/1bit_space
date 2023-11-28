use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct MovementAttributes {
    #[export]
    impulse: f64,
    #[export]
    turn_speed: f64,
    #[export]
    max_speed: f64,

    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl IResource for MovementAttributes {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            impulse: 30.0,
            turn_speed: 2.0,
            max_speed: 500.0,
            base
        }
    }
}