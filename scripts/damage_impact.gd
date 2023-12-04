class_name  ImpactDamage
extends Object

var damage: int
var normal: Vector2
var from: Node2D

func _init(dmg: int, from_node: Node2D,) -> void:
	from = from_node
	damage = dmg

