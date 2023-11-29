use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

pub struct ThrottleData {
    pub current_velocity: Vector2,
    pub current_direction: Vector2,
    pub throttle: f64,
}

impl ThrottleData {
    pub fn get_new_velocity(&self, impulse: f64) -> Vector2 {
        return self.current_velocity + (self.current_direction * (self.throttle * impulse) as f32);
    }
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Ship {
    #[export]
    rotation_direction: f64,
    #[export]
    forward_throttle: f64,
    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Ship {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            rotation_direction: 0.0,
            forward_throttle: 0.0,
            base
        }
    }
}

#[godot_api]
impl Ship {
    pub fn get_throttle_data(&self) -> ThrottleData {
        let current_velocity = self.base.get_velocity();
        let current_direction = Vector2::UP.rotated(self.base.get_rotation());

        ThrottleData {
            current_velocity,
            current_direction,
            throttle: self.forward_throttle,
        }
    }
}
