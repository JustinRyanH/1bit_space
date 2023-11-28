use godot::engine::{CharacterBody2D, GpuParticles2D, ICharacterBody2D, INode, InputEvent};
use godot::prelude::*;

use crate::prelude::*;


#[derive(GodotClass)]
#[class(base=Node)]
struct PlayerShipInput {
    #[export]
    ship_movement: Option<Gd<ShipMovement>>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for PlayerShipInput {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            ship_movement: None,
        }
    }

    fn process(&mut self, delta: f64) {
        let Some(mut movement) = self.ship_movement.clone() else { return; };
        let mut movement = movement.bind_mut();
        let input = Input::singleton();
        let rotate_axis = input.get_axis("Rotate Left".into(), "Rotate Right".into());
        movement.set_rotation_direction(rotate_axis as f64);
    }
}

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
        let turn_speed = movement_attributes.get_turn_speed();
        let rotation = actor.get_rotation();

        actor.set_rotation(rotation + (self.rotation_direction * delta * turn_speed) as f32);
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
        self.move_ship(delta);
    }
}

#[godot_api]
impl Ship {
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
