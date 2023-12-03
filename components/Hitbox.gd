class_name Hitbox
extends Area2D

@export var damage: int
var disabled := false

func _init() -> void:
	collision_layer = 2
	collision_mask = 0
