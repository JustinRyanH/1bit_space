use godot::prelude::*;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ShipProjectileSystem {
    #[export]
    project_bus: Gd<BulletSpawnBus>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ShipProjectileSystem {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            project_bus: BulletSpawnBus::new_gd(),
        }
    }
}