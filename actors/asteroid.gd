extends Node2D


func _on_hurtbox_component_hurt(hitbox: HitboxComponent) -> void:
	print(hitbox.get_parent().name)
