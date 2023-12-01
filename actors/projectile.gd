extends Node2D

@export var projectile_stats: ProjectileStates

@onready var actor_move_component := $ActorMoveComponent as ActorMoveComponent
@onready var destroy_component: = $DestroyComponent as DestoryComponent

func _ready() -> void:
	actor_move_component.speed = projectile_stats.speed

func _on_hurtbox_component_hurt(hitbox: HitboxComponent) -> void:
	destroy_component.destroy()
