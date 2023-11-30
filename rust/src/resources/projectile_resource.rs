use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct ProjectileResource {
    #[export]
    speed: f64,

    #[export]
    damage: f64,

    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ProjectileResource {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            speed: 750.0,
            damage: 10.0,
            base
        }
    }
}
