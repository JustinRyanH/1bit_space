use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

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
impl Ship { }
