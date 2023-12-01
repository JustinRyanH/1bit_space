use godot::prelude::*;

use crate::prelude::*;

#[derive(Clone)]
pub struct Movement {
    movement_stats: Gd<MovementStats>,
}

impl Movement {
    pub fn new(movement_stats: Gd<MovementStats>) -> Self {
        return Self { movement_stats: movement_stats };

    }

    pub fn update_movement_stats(&mut self, movement_stats: &Gd<MovementStats>) {
        self.movement_stats = movement_stats.clone();
    }

    pub fn rotate_ship(&self, ship: &mut Ship, delta: f64) {
        let throttle_data = ship.get_throttle_data();
        let movement_state = self.movement_stats.bind().get_movement_state();

        let new_rotation = throttle_data.get_new_rotation(delta, &movement_state);

        ship.base.set_rotation(new_rotation);
    }

    pub fn move_ship_forward(&self, ship: &mut Ship, delta: f64) {
        let movement_state = self.movement_stats.bind().get_movement_state();
        let throttle_data = ship.get_throttle_data();
        let new_velocity = throttle_data.get_new_velocity(delta, &movement_state);

        ship.base.set_velocity(new_velocity);
    }
}
