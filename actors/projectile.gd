extends Node2D

@export var projectile_stats: ProjectileStates
@onready var actor_move_component := $ActorMoveComponent as ActorMoveComponent

func _ready() -> void:
	actor_move_component.speed = projectile_stats.speed
