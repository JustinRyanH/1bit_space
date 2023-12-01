class_name ActorMoveComponent
extends Node

@export var actor: Node2D
@export var speed: float = 300

func _process(delta: float) -> void:
	var velocity = Vector2.UP.rotated(actor.rotation) * delta * speed
	actor.translate(velocity)
