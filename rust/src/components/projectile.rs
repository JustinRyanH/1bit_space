use godot::engine::Timer;
use godot::prelude::*;


#[derive(GodotClass)]
#[class(base=Node)]
pub struct ProjectileComponent {
    #[export]
    lifetime_timer: Option<Gd<Timer>>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ProjectileComponent {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            lifetime_timer: None,
        }
    }
}
