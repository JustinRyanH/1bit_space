use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;
use crate::components::ActorMovementComponent;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct ProjectileActor {
    #[export]
    resource: Gd<ProjectileResource>,
    #[export]
    movement_actor: Option<Gd<ActorMovementComponent>>,
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for ProjectileActor {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            resource: ProjectileResource::new_gd(),
            movement_actor: None,
            base,
        }
    }
}
