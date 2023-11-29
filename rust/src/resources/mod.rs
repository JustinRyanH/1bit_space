use godot::engine::Resource;
use godot::prelude::*;

pub mod movement_attributes;

pub use movement_attributes::MovementAttributes;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct BulletSpawnBus {
    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl IResource for BulletSpawnBus {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

#[godot_api]
impl BulletSpawnBus {
    #[signal]
    fn spawn_bullet(scene_path: String, position: Vector2, rotation: f64) {}
}