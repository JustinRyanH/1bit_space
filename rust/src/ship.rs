use godot::engine::{CharacterBody2D, GpuParticles2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Ship {
    #[export]
    linear_speed: f64,
    #[export]
    angular_speed: f64,
    #[export]
    max_speed: f32,

    engine_particles: Option<Gd<GpuParticles2D>>,
    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Ship {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            linear_speed: 150.0,
            angular_speed: 5.0,
            max_speed: 500.0,
            engine_particles: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.engine_particles = self.base.try_get_node_as::<GpuParticles2D>("RearEngineParticles");
        self.toggle_engine(false);
    }

    fn physics_process(&mut self, delta: f64) {
        self.rotate_ship(delta);
        self.move_ship(delta);
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

    fn move_ship(&mut self, delta: f64) {
        let input = Input::singleton();
        let movement_axis = input.get_action_strength("Accelerate".into());
        let base_velocity = self.base.get_velocity();
        if movement_axis > 0.0 {
            let velocity_direction = Vector2::UP.rotated(self.base.get_rotation());
            let new_velocity = base_velocity + (velocity_direction * movement_axis * (delta * self.linear_speed) as f32);
            let new_velocity = new_velocity.limit_length(self.max_speed.into());

            self.base.set_velocity(new_velocity);
            self.toggle_engine(true);
        } else {
            self.toggle_engine(false);
        }
        self.base.move_and_slide();
    }

    fn toggle_engine(&mut self, value: bool) {
        let Some(mut particles) = self.engine_particles.clone() else { return; };
        particles.set_emitting(value);
    }
}
