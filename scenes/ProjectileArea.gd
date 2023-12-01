extends Area2D

@export var actor: Node2D

func wrap_to(new_location: Vector2) -> void:
	actor.global_position = new_location
