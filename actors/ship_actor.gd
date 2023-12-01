class_name ShipActor
extends CharacterBody2D

@export var forward_throttle: float
@export var rotation_throttle: float
@export var configuration: ShipConfiguration


func wrap_to(new_location: Vector2):
	global_position = new_location
