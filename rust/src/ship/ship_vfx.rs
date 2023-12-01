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

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ShipVfxSystem {
    #[export]
    rear_engine: Option<Gd<GpuParticles2D>>,
    #[export]
    ship: Option<Gd<Ship>>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ShipVfxSystem {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            rear_engine: None,
            ship: None,
            base
        }
    }

    fn process(&mut self, _delta: f64) {
        let Some(ship) = self.ship.clone() else {
            godot_warn!("No Ship Found. ShipVfxSystem will not run");
            return;
        };
        if self.update_engine(ship) { return; }
    }
}

#[godot_api]
impl ShipVfxSystem {
    fn update_engine(&mut self, ship: Gd<Ship>) -> bool {
        let Some(mut rear_engine) = self.rear_engine.clone() else {
            godot_warn!("No Engine Particles Found. ShipVfxSystem will not run");
            return true;
        };
        if ship.bind().get_forward_throttle() > 0.0 {
            rear_engine.set_emitting(true);
        } else {
            rear_engine.set_emitting(false);
        }
        false
    }
}

