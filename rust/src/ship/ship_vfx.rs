use godot::engine::GpuParticles2D;
use godot::prelude::*;

use crate::prelude::*;

#[derive(Clone)]
pub struct ShpVFX {
    pub rear_engine: Gd<GpuParticles2D>,
}

impl ShpVFX {
    pub fn update_engine(&mut self, ship: &Ship) -> bool {
        if ship.get_forward_throttle() > 0.0 {
            self.rear_engine.set_emitting(true);
        } else {
            self.rear_engine.set_emitting(false);
        }
        false
    }
}

