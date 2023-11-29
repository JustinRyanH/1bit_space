use godot::bind::{godot_api, GodotClass};
use godot::obj::Base;
use godot::engine::{IResource, Resource};
use godot::builtin::Vector2;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct ProjectileMessageBus {
    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ProjectileMessageBus {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

#[godot_api]
impl ProjectileMessageBus {
    #[signal]
    fn spawn_bullet(scene_path: String, position: Vector2, rotation: f64) {}
}
