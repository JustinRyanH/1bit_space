extends Node

enum Direction {
	CounterClockwise = -1,
	Clockwise = 1,
}

## The Actor to Rotate
@export var actor: Node2D
## Number of rotations per second
@export var speed: float
## The Diration of the rotation. 
@export var direction: Direction = Direction.Clockwise

func _process(delta: float) -> void:
	actor.rotate(delta * (2 * PI * speed) * direction)
