use godot::engine::{INode, Node};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ShipMovementSystem {
    #[export]
    ship: Option<Gd<Ship>>,
    #[export]
    movement_attributes: Gd<MovementAttributes>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ShipMovementSystem {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            ship: None,
            movement_attributes: MovementAttributes::new_gd(),
        }
    }

    fn process(&mut self, delta: f64) {
        self.rotate_ship(delta);
        self.move_ship_forward(delta);
    }

    fn physics_process(&mut self, _delta: f64) {
        let Some(mut ship) = self.ship.clone() else { return; };
        ship.move_and_slide();
    }
}


#[godot_api]
impl ShipMovementSystem {
    fn rotate_ship(&mut self, delta: f64) {
        let Some(mut ship) = self.ship.clone() else { return; };
        let throttle_data = ship.bind().get_throttle_data();
        let movement_state = self.movement_attributes.bind().get_movement_state();

        let new_rotation = throttle_data.get_new_rotation(delta, &movement_state);

        ship.set_rotation(new_rotation);
    }

    fn move_ship_forward(&mut self, delta: f64) {
        let Some(mut ship) = self.ship.clone() else { return; };

        let movement_state = self.movement_attributes.bind().get_movement_state();
        let throttle_data = ship.bind().get_throttle_data();
        let new_velocity = throttle_data.get_new_velocity(delta, &movement_state);

        ship.set_velocity(new_velocity);
    }
}
