use godot::engine::{INode, Input, Node};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerShipInput {
    #[export]
    ship_movement: Option<Gd<ShipMovement>>,
    #[export]
    ship: Option<Gd<Ship>>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for PlayerShipInput {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            ship_movement: None,
            ship: None,
        }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();


        let acceleration = input.get_action_strength("Accelerate".into());
        let rotate_axis = input.get_axis("Rotate Left".into(), "Rotate Right".into());

        self.update_movement(acceleration, rotate_axis);
        self.update_ship(acceleration, rotate_axis);
    }
}

#[godot_api]
impl PlayerShipInput {
    fn update_movement(&mut self, _acceleration: f32, rotate_axis: f32) {
        let Some(mut movement) = self.ship_movement.clone() else { return;  };
        let mut movement = movement.bind_mut();
        movement.set_rotation_direction(rotate_axis as f64);
    }

    fn update_ship(&mut self, acceleration: f32, rotate_axis: f32) {
        let Some(mut ship) = self.ship.clone() else { return; };
        let mut ship = ship.bind_mut();
        ship.set_rotation_direction(rotate_axis as f64);
        ship.set_forward_throttle(acceleration as f64);
    }
}
