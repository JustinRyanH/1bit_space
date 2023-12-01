extends Node2D

@export var projectile_stats: ProjectileStates

@onready var actor_move_component := $ActorVelocityComponent as ActorVelocityComponent
@onready var destroy_component: = $DestroyComponent as DestoryComponent
@onready var hitbox_component: = $HitboxComponent as HitboxComponent


func _ready() -> void:
	hitbox_component.damage = projectile_stats.damage
	var speed: float = projectile_stats.speed
	var velocity = Vector2.UP.rotated(rotation) * speed
	actor_move_component.velocity = velocity

func _on_hurtbox_component_hurt(hitbox: HitboxComponent) -> void:
	destroy_component.destroy()
