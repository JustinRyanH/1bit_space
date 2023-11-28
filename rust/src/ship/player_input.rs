use godot::bind::{godot_api, GodotClass};
use godot::obj::{Base, Gd};
use godot::engine::{INode, Input, Node};
use crate::ship::movement::ShipMovement;

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
