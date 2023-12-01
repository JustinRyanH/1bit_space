use godot::engine::{CharacterBody2D, GpuParticles2D, ICharacterBody2D};
use godot::prelude::*;
use crate::prelude::MovementState;
use crate::ship::ship_vfx::ShipVfxSystemV2;

pub struct ThrottleData {
    pub current_velocity: Vector2,
    pub current_direction: Vector2,
    pub throttle: f64,
    pub rotation_direction: f64,
    pub current_rotation: f64,
}

impl ThrottleData {
    pub fn get_new_velocity(&self, dt: f64, movement_state: &MovementState) -> Vector2 {
        let impulse = movement_state.impulse * dt;
        let move_speed = movement_state.max_speed as f32;

        let new_velocity_unlimited = self.current_velocity + (self.current_direction * (self.throttle * impulse) as f32);

        new_velocity_unlimited.limit_length(move_speed.into())
    }

    pub fn get_new_rotation(&self, dt: f64, movement_state: &MovementState) -> f32 {
        (self.current_rotation + self.rotation_direction * dt * movement_state.turn_speed) as f32
    }
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Ship {
    #[export]
    rotation_direction: f64,
    #[export]
    forward_throttle: f64,

    vfx: Option<ShipVfxSystemV2>,
    #[export]
    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Ship {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            rotation_direction: 0.0,
            forward_throttle: 0.0,
            vfx: None,
            base
        }
    }

    fn ready(&mut self) {
        let Some(rear_engine) = self.base.try_get_node_as::<GpuParticles2D>("RearEngineParticles") else { return; };
        self.vfx = Some(ShipVfxSystemV2 { rear_engine });
    }

    fn process(&mut self, _delta: f64) {
        if let Some(mut vfx) = self.vfx.clone() {
            vfx.update_engine(self);
        }
    }
}

#[godot_api]
impl Ship {
    pub fn get_throttle_data(&self) -> ThrottleData {
        let current_velocity = self.base.get_velocity();
        let current_direction = Vector2::UP.rotated(self.base.get_rotation());

        ThrottleData {
            current_velocity,
            current_direction,
            throttle: self.forward_throttle,
            rotation_direction: self.rotation_direction,
            current_rotation: self.base.get_rotation() as f64,
        }
    }
}
