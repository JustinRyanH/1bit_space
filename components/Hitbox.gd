class_name Hitbox
extends Area2D

var ignore_targets: Array = []

@export var damage: int
var disabled := false

func _init() -> void:
	collision_layer = 2
	collision_mask = 0


func get_impact_damage() -> BasicDamage:
	if owner.has_method("get_impact_damage"):
		return owner.get_impact_damage()
	else:
		return BasicDamage.new(damage, owner)