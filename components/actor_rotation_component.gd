class_name ActorRotationComponent
extends Node

## The Actor to Rotate
@export var actor: Node2D
## Number of rotations per second
@export var speed: float
## The Diration of the rotation. 
@export_range(-1, 1) var direction: float = 1

func _process(delta: float) -> void:
	actor.rotate(delta * (2 * PI * speed) * direction)
