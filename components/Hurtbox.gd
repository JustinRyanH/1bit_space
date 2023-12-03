class_name Hurtbox
extends Area2D

var disabled := false

func _init() -> void:
	collision_layer = 0
	collision_mask = 2

func _ready() -> void:
	area_entered.connect(_on_area_entered)

func _on_area_entered(hitbox: Hitbox) -> void:
	if disabled: return
	if hitbox == null: return
	if hitbox.owner == owner: return
	if hitbox.disabled: return
	
	if owner.has_method("take_damage"):
		owner.take_damage(hitbox.damage)
