class_name Hurtbox
extends Area2D

var disabled := false
var ignore_targets: Array = []

func _init() -> void:
	collision_layer = 0
	collision_mask = 2

func _ready() -> void:
	area_entered.connect(_on_area_entered)

func _on_area_entered(hitbox: Hitbox) -> void:
	if disabled: return
	if hitbox == null: return
	if hitbox.owner == owner: return
	if hitbox.ignore_targets.has(owner): return
	if ignore_targets.has(hitbox.owner): return
	if hitbox.disabled: return
	
	if owner.has_method("take_damage_v2"):
		var impact_damage = ImpactDamage.new(hitbox.damage, hitbox.owner)
		owner.take_damage_v2(impact_damage)
