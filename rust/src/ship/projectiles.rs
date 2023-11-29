use godot::prelude::*;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ShipProjectileSystem {
    #[export]
    projectile_message_bus: Gd<BulletSpawnBus>,
    #[export]
    ship: Option<Gd<Ship>>,
    #[export]
    start_location: Option<Gd<Node2D>>,
    #[export]
    projectile_scene: Option<Gd<PackedScene>>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ShipProjectileSystem {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            ship: None,
            start_location: None,
            projectile_scene: None,
            projectile_message_bus: BulletSpawnBus::new_gd(),
        }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        if input.is_action_just_pressed("Fire".into()) {
            self.fire_projectile();
        }
    }
}

#[godot_api]
impl ShipProjectileSystem {
    fn fire_projectile(&mut self) {
        let Some(projectile_scene) = self.projectile_scene.clone() else { return; };
        let Some(start_location) = self.start_location.clone() else { return; };
        let Some(ship) = self.ship.clone() else { return; };
        let mut project_bus = self.projectile_message_bus.clone();

        if project_bus.has_signal("spawn_bullet".into()) {
            let start_location = start_location.get_global_position();
            let rotation = ship.get_rotation();
            let scene_path = projectile_scene.get_path();
            let args = &[Variant::from(scene_path), Variant::from(start_location), Variant::from(rotation)];
            project_bus.emit_signal("spawn_bullet".into(), args);
        }
    }

}