use godot::engine::{GpuParticles2D, Sprite2D, Timer};
use godot::prelude::*;
use crate::components::ActorMovementComponent;


#[derive(GodotClass)]
#[class(base=Node)]
pub struct ProjectileComponent {
    #[export]
    lifetime_timer: Option<Gd<Timer>>,
    #[export]
    tail: Option<Gd<GpuParticles2D>>,
    #[export]
    bullet_die: Option<Gd<GpuParticles2D>>,
    #[export]
    movement: Option<Gd<ActorMovementComponent>>,
    #[export]
    sprite: Option<Gd<Sprite2D>>,
    #[export]
    actor: Option<Gd<Node>>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for ProjectileComponent {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            lifetime_timer: None,
            tail: None,
            bullet_die: None,
            movement: None,
            sprite: None,
            actor: None,
        }
    }

    fn ready(&mut self) {
        let Some(mut lifetime_timer) = self.lifetime_timer.clone() else { return; };
        let finish_timer = Callable::from_object_method(&self.base, "finish_timer");
        lifetime_timer.connect("timeout".into(), finish_timer);
        lifetime_timer.start();
    }
}

#[godot_api]
impl ProjectileComponent {
    #[func]
    fn finish_timer(&mut self) {
        let Some(mut particles) = self.tail.clone() else { return; };
        let Some(mut movement) = self.movement.clone() else { return; };
        let Some(mut bullet_die) = self.bullet_die.clone() else { return; };

        let bullet_die_lifetime = bullet_die.get_lifetime();
        movement.bind_mut().set_speed(0.0);
        particles.set_emitting(false);
        bullet_die.set_emitting(true);

        let cleanup_method = Callable::from_object_method(&self.base, "cleanup");
        bullet_die.connect("finished".into(), cleanup_method);

        self.tween_out_sprite(bullet_die_lifetime);
    }

    fn tween_out_sprite(&mut self, bullet_die_lifetime: f64) {
        let Some(sprite) = self.sprite.clone() else { return; };
        if let Some(mut tween) = self.base.create_tween() {
            let mut modulate = sprite.get_modulate();
            modulate.a = 0.0;

            tween.tween_property(sprite.clone().upcast(), "modulate".into(), Variant::from(modulate), bullet_die_lifetime);
        }
    }

    #[func]
    fn cleanup(&mut self) {
        let Some(mut actor) = self.actor.clone() else { return; };
        actor.queue_free();
    }
}
