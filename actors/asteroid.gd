extends Node2D

@export var health: float = 1:
	set(value):
		health = value
		if health <= 0:
			destroy()
	get:
		return health

func destroy() -> void:
	queue_free()

func _on_hurtbox_component_hurt(hitbox: HitboxComponent) -> void:
	health -= hitbox.damage
