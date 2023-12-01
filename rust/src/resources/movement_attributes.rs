use godot::engine::Resource;
use godot::prelude::*;

pub struct MovementState {
    pub impulse: f64,
    pub turn_speed: f64,
    pub max_speed: f64,
}

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct MovementStats {
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
impl IResource for MovementStats {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            impulse: 30.0,
            turn_speed: 2.0,
            max_speed: 500.0,
            base
        }
    }
}

#[godot_api]
impl MovementStats {
    pub fn get_movement_state(&self) -> MovementState {
        MovementState {
            impulse: self.impulse,
            turn_speed: self.turn_speed,
            max_speed: self.max_speed,
        }
    }
}