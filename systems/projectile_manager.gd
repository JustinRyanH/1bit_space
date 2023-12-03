extends Node2D

@export var projectile_bus: ProjectileBus

func _ready():
	projectile_bus.create_projectile.connect(spawn_projectile)
	projectile_bus.add_projectile_to_world.connect(insert_projectile)
	
func spawn_projectile(scene: PackedScene, pos: Vector2, rot: float, _base_speed: float):
	var projectile := scene.instantiate() as Projectile
	projectile.global_position = pos
	projectile.rotation = rot
	add_child(projectile)

func insert_projectile(new_projectile: Node2D) -> void:
	var projectile := new_projectile as Projectile;
	if projectile:
		add_child(projectile)

