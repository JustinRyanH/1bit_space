extends Node2D

@export var projectile_bus: ProjectileBus

func _ready():
	projectile_bus.create_projectile.connect(spawn_projectile)
	
func spawn_projectile(scene: PackedScene, position: Vector2, rotation: float, base_speed: float):
	var projectile := scene.instantiate() as Projectile
	projectile.global_position = position
	projectile.rotation = rotation
	projectile.speed_modifier = base_speed
	add_child(projectile)
