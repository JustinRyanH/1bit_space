use godot::engine::{CharacterBody2D, INode, Node};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ShipMovement {
    #[export]
    rotation_direction: f64,
    #[export]
    forward_throttle: f64,

    #[export]
    actor: Option<Gd<CharacterBody2D>>,
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

        self.rotate_ship(delta, &mut actor);
        self.move_ship_forward(delta, &mut actor);
    }
}


#[godot_api]
impl ShipMovement {
    fn rotate_ship(&mut self, delta: f64, actor: &mut Gd<CharacterBody2D>) {
        let movement_attributes = self.movement_attributes.bind();
        let turn_speed = movement_attributes.get_turn_speed();
        let rotation = actor.get_rotation();

        actor.set_rotation(rotation + (self.rotation_direction * delta * turn_speed) as f32);
    }

    fn move_ship_forward(&mut self, delta: f64, actor: &mut Gd<CharacterBody2D>) {
        let movement_attributes = self.movement_attributes.bind();
        let max_velocity = movement_attributes.get_max_speed() as f32;
        let impulse = movement_attributes.get_impulse();

        let base_velocity = actor.get_velocity();
        let velocity_direction = Vector2::UP.rotated(actor.get_rotation());

        let new_velocity = base_velocity + (velocity_direction * (self.forward_throttle * delta * impulse) as f32);
        let new_velocity = new_velocity.limit_length(max_velocity.into());

        actor.set_velocity(new_velocity);
    }
}
