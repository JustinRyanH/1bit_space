use godot::engine::{CharacterBody2D, GpuParticles2D, ICharacterBody2D, Input, NodeExt};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Ship {
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
        self.move_ship(delta);
    }
}

#[godot_api]
impl Ship {
    fn move_ship(&mut self, _delta: f64) {
        let input = Input::singleton();
        let movement_axis = input.get_action_strength("Accelerate".into());
        if movement_axis > 0.0 {
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
