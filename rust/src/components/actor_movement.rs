use godot::bind::{godot_api, GodotClass};
use godot::obj::{Base, Gd};
use godot::engine::{INode, Node, Node2D};
use godot::builtin::Vector2;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ActorMovementComponent {
    #[export]
    actor: Option<Gd<Node2D>>,
    #[export]
    speed: f64,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ActorMovementComponent {
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
