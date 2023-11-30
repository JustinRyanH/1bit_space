use godot::bind::{godot_api, GodotClass};
use godot::obj::{Base, Gd, UserClass};
use godot::engine::{INode, load, Node, Node2D, PackedScene};
use godot::builtin::{Callable, Vector2};
use godot::log::godot_error;
use godot::prelude::ToGodot;
use crate::prelude::ProjectileMessageBus;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct ProjectileManager {
    #[export]
    bullet_event_bus: Gd<ProjectileMessageBus>,
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl INode for ProjectileManager {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base, bullet_event_bus: ProjectileMessageBus::new_gd() }
    }

    fn ready(&mut self) {
        let callable = Callable::from_object_method(&self.base.to_godot(), "spawn_bullet");
        self.bullet_event_bus.connect("spawn_bullet".into(), callable);
    }
}

#[godot_api]
impl ProjectileManager {
    #[func]
    pub fn spawn_bullet(&mut self, scene_path: String, position: Vector2, rotation: f32) {
        let scene = load::<PackedScene>(&scene_path);
        if let Some(scene) = scene.instantiate() {
            let mut bullet = scene.cast::<Node2D>();
            bullet.set_global_position(position);
            bullet.set_rotation(rotation);
            self.base.add_child(bullet.upcast::<Node>());
        } else {
            godot_error!("Could not build: {:?}", scene_path);
        }
    }
}