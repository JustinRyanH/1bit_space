use godot::engine::Node2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ActorMovement {
    #[export]
    actor: Option<Gd<Node2D>>,
    #[export]
    speed: f64,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ActorMovement {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            actor: None,
            speed: 1.0,
        }
    }

    fn process(&mut self, delta: f64) {
        let Some(mut actor) = self.actor.clone() else { return; };
        let rotation = actor.get_rotation();
        let direction = Vector2::UP.rotated(rotation);

        let position = actor.get_position();
        let offset = direction * (self.speed * delta) as f32;
        actor.set_position(position + offset);
    }
}