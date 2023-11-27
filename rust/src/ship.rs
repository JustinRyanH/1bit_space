use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Ship {
    speed: f64,
    angular_speed: f64,

    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Ship {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            speed: 400.0,
            angular_speed: 100.0,
            base,
        }
    }
}
