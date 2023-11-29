extends Node

@export var actor: Node2D
@export var bullet_spawn_bus: BulletSpawnBus
@export var bullet_scene: PackedScene
@export var gun_location: Node2D

func _process(delta):
	if Input.is_action_just_pressed("Fire"):
		bullet_spawn_bus.emit_signal("spawn_bullet", bullet_scene, gun_location.global_position, actor.rotation)
