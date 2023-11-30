class_name ShipPlayerInput
extends Node

@export var ship_actor: ShipActor

func _process(_delta: float) -> void:
	ship_actor.rotation_throttle = Input.get_axis("Rotate Left", "Rotate Right")
	ship_actor.forward_throttle = Input.get_action_strength("Accelerate")
