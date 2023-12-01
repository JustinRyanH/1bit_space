class_name ActorVelocityComponent
extends Node

@export var actor: Node2D
@export var velocity: Vector2 = Vector2.ZERO

func _process(delta: float) -> void:
	actor.translate(velocity)
