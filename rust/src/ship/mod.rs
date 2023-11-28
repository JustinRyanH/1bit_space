use godot::engine::{CharacterBody2D, ICharacterBody2D, INode};
use godot::prelude::*;

use crate::prelude::*;

mod ship;


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

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        let Some(mut movement) = self.ship_movement.clone() else { return; };
        let mut movement = movement.bind_mut();

        let acceleration = input.get_action_strength("Accelerate".into());
        let rotate_axis = input.get_axis("Rotate Left".into(), "Rotate Right".into());

        movement.set_rotation_direction(rotate_axis as f64);
        movement.set_throttle(acceleration as f64);
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
struct ShipMovement {
    rotation_direction: f64,
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


#[godot_api]
impl ShipMovement {
    pub fn set_rotation_direction(&mut self, direction: f64) {
        self.rotation_direction = direction;
    }

    pub fn set_throttle(&mut self, throttle: f64) {
        self.forward_throttle = throttle;
    }

    fn rotate_ship(&mut self, delta: f64, actor: &mut Gd<CharacterBody2D>) {
        let movement_attributes = self.movement_attributes.bind();
        let turn_speed = movement_attributes.get_turn_speed();
        let rotation = actor.get_rotation();

        actor.set_rotation(rotation + (self.rotation_direction * delta * turn_speed) as f32);
    }
}
