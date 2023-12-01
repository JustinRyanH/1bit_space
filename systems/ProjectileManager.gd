extends Node2D

@export var projectile_bus: ProjectileBus


func _ready():
	projectile_bus.create_projectile.connect(spawn_projectile)
	
func spawn_projectile(scene: PackedScene, position: Vector2, rotation: float):
	var projectile: Node2D = scene.instantiate() as Node2D
	projectile.global_position = position
	projectile.rotation = rotation
	add_child(projectile)
