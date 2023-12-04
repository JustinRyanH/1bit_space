class_name  BasicDamage
extends Object

var damage: int
var direction: Vector2 = Vector2.ZERO
var position: Vector2 = Vector2.ZERO
var from: Node2D

func _init(dmg: int, from_node: Node2D,) -> void:
	from = from_node
	damage = dmg

