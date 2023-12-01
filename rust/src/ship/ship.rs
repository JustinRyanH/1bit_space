use godot::engine::{CharacterBody2D, GpuParticles2D, ICharacterBody2D};
use godot::prelude::*;

use crate::prelude::*;

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
    #[export]
    movement_stats: Gd<MovementAttributes>,

    vfx: Option<ShpVFX>,
    ship_movement: Movement,
    #[export]
    #[base]
    pub base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Ship {
    fn init(base: Base<Self::Base>) -> Self {
        let movement_stats = MovementAttributes::new_gd();
        Self {
            rotation_direction: 0.0,
            forward_throttle: 0.0,
            movement_stats: movement_stats.clone(),
            ship_movement: Movement::new(movement_stats),
            vfx: None,
            base
        }
    }

    fn ready(&mut self) {
        let Some(rear_engine) = self.base.try_get_node_as::<GpuParticles2D>("RearEngineParticles") else { return; };
        self.vfx = Some(ShpVFX { rear_engine });
        self.ship_movement.update_movement_stats(&self.movement_stats);
    }

    fn process(&mut self, delta: f64) {
        self.gather_input();

        if let Some(mut vfx) = self.vfx.clone() {
            vfx.update_engine(self);
        }

        let ship_movement = self.ship_movement.clone();
        ship_movement.rotate_ship(self, delta);
        ship_movement.move_ship_forward(self, delta);
        self.base.move_and_slide();
    }
}

#[godot_api]
impl Ship {
    pub fn gather_input(&mut self) {
        let input = Input::singleton();
        self.forward_throttle = input.get_action_strength("Accelerate".into()) as f64;
        self.rotation_direction = input.get_axis("Rotate Left".into(), "Rotate Right".into()) as f64;
    }
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
