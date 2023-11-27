use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Ship {
    #[export]
    speed: f64,
    #[export]
    angular_speed: f64,

    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Ship {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            speed: 2.0,
            angular_speed: 5.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.rotate_ship(delta);
    }
}

#[godot_api]
impl Ship {
    fn rotate_ship(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut base = self.base.clone();
        let rotate_axis = input.get_axis("Rotate Left".into(), "Rotate Right".into());

        let base_rotation = base.get_rotation();
        base.set_rotation(base_rotation + (rotate_axis * (delta * self.angular_speed) as f32));
    }
}
