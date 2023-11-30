use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=Node)]
pub struct ProjectileSystem {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ProjectileSystem {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base}
    }

    fn process(&mut self, delta: f64) {
    }
}
