class_name HitboxComponent
extends Area2D

@export var damage: float = 1.0

signal hit_hurtbox(hurtbox: HurtboxComponent)

func _ready() -> void:
	area_entered.connect(_on_hurtbox_entered)

func _on_hurtbox_entered(hurtbox: HurtboxComponent) -> void:
	if not hurtbox is HurtboxComponent: return
	if hurtbox.get_parent() == get_parent(): return
	if hurtbox.is_invincible: return
	
	hit_hurtbox.emit(hurtbox)
	hurtbox.hurt.emit(self)
