class_name HitboxComponent
extends Area2D

@export var damage: float = 1.0
@export var actor: Node2D

signal hit_hurtbox(hurtbox: HurtboxComponent, positon: Vector2)

func _ready() -> void:
	area_entered.connect(_on_hurtbox_entered)

func _on_hurtbox_entered(hurtbox: HurtboxComponent) -> void:
	if not hurtbox is HurtboxComponent: return
	if hurtbox.is_invincible: return
	
	hit_hurtbox.emit(hurtbox, actor.global_position)
	hurtbox.hurt.emit(self)
