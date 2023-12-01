extends Node2D

@export var speed_low: float = 10.0
@export var speed_high: float = 200.0
@export var health: float = 1:
	set(value):
		health = value
		if health <= 0:
			destroy()
	get:
		return health

@onready var actor_velocity_component:  = $ActorVelocityComponent as ActorVelocityComponent


func _ready() -> void:
	var initial_direction = Vector2(randf_range(-1, 1), randf_range(-1, 1)).normalized()
	var random_speed = randf_range(speed_low, speed_high)
	actor_velocity_component.velocity = initial_direction * random_speed

func destroy() -> void:
	queue_free()

func _on_hurtbox_component_hurt(hitbox: HitboxComponent) -> void:
	health -= hitbox.damage
