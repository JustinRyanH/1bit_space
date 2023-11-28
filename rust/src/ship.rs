use godot::engine::{CharacterBody2D, GpuParticles2D, ICharacterBody2D, INode};
use godot::prelude::*;

use crate::prelude::*;


#[derive(GodotClass)]
#[class(base=Node)]
struct ShipMovement {
    rotation_direction: f64,
    forward_throttle: f64,

    #[export]
    actor: Option<Gd<Node2D>>,
    #[export]
    movement_attributes: Gd<MovementAttributes>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ShipMovement {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            rotation_direction: 0.0,
            forward_throttle: 0.0,
            actor: None,
            movement_attributes: MovementAttributes::new_gd(),
        }
    }

    fn process(&mut self, delta: f64) {
        let Some(mut actor) = self.actor.clone() else { return; };
        let movement_attributes = self.movement_attributes.bind();
        let angular_speed = movement_attributes.get_turn_speed();
        let rotation = actor.get_rotation();
        actor.set_rotation(rotation + (self.rotation_direction * delta * angular_speed) as f32);
    }
}

#[godot_api]
impl ShipMovement {
    pub fn set_rotation_direction(&mut self, direction: f64) {
        self.rotation_direction = direction;
    }
}


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
