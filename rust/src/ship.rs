use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Ship {
    #[export]
    linear_speed: f64,
    #[export]
    angular_speed: f64,

    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Ship {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            linear_speed: 2.0,
            angular_speed: 5.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.rotate_ship(delta);
        let input = Input::singleton();
        let movement_axis = input.get_action_strength("Accelerate".into());
        let base_velocity = self.base.get_velocity();
        if movement_axis > 0.0 {
            let velocity_direction = Vector2::UP.rotated(self.base.get_rotation());
            let new_velocity = base_velocity + (velocity_direction * movement_axis * (delta * self.linear_speed) as f32);
            godot_print!("new_velocity: {:?}", new_velocity);
            self.base.set_velocity(new_velocity);
        }
        self.base.move_and_slide();
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
