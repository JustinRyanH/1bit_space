class_name GunFireComponent
extends Node

@export var projectile_scene: PackedScene
@export var projectile_bus: ProjectileBus
@export var gun_marker: Marker2D

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if Input.is_action_just_pressed("Fire"):
		projectile_bus.create_projectile.emit(projectile_scene, gun_marker.global_position, gun_marker.global_rotation)
