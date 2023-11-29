use godot::engine::{GpuParticles2D, INode, Node};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ShipMovement {
    #[export]
    rotation_direction: f64,
    #[export]
    engine_particles: Option<Gd<GpuParticles2D>>,

    #[export]
    actor: Option<Gd<Ship>>,
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
            actor: None,
            engine_particles: None,
            movement_attributes: MovementAttributes::new_gd(),
        }
    }

    fn process(&mut self, delta: f64) {
        self.rotate_ship(delta);
        self.move_ship_forward(delta);
        self.toggle_engine();
    }

    fn physics_process(&mut self, _delta: f64) {
        let Some(mut actor) = self.actor.clone() else { return; };
        actor.move_and_slide();
    }
}


#[godot_api]
impl ShipMovement {
    fn rotate_ship(&mut self, delta: f64) {
        let Some(mut actor) = self.actor.clone() else { return; };
        let movement_attributes = self.movement_attributes.bind();
        let turn_speed = movement_attributes.get_turn_speed();
        let rotation = actor.get_rotation();

        actor.set_rotation(rotation + (self.rotation_direction * delta * turn_speed) as f32);
    }

    fn move_ship_forward(&mut self, delta: f64) {
        let Some(mut actor) = self.actor.clone() else { return; };

        let movement_attributes = self.movement_attributes.bind();
        let max_velocity = movement_attributes.get_max_speed() as f32;
        let impulse = movement_attributes.get_impulse();

        let throttle_data = actor.bind().get_throttle_data();
        let new_velocity = throttle_data.get_new_velocity(delta * impulse);
        let new_velocity = new_velocity.limit_length(max_velocity.into());


        actor.set_velocity(new_velocity);
    }

    fn toggle_engine(&mut self) {
        let Some(ship) = self.actor.clone() else { return; };
        let Some(mut engine_particles) = self.engine_particles.clone() else {
            godot_error!("No Engine for Ship: {:?}", self.actor);
            return;
        };
        engine_particles.set_emitting(ship.bind().get_forward_throttle() > 0.0);
    }
}
