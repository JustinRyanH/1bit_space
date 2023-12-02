class_name AsteroidManager
extends Node2D

@export var asteroid_spawn_bus: AsteroidSpawnBus

func _ready() -> void:
	asteroid_spawn_bus.spawn_asteroid.connect(spawn_sub_asteroid)

func spawn_sub_asteroid(scene: PackedScene, location: Vector2, velocity: Vector2, count: int) -> void:
	var start = -(count / 2) 
	var angle = (PI / 4)
	var length = velocity.length()
	var initial_direction = velocity.normalized();
	for i in range(start, count):
		if i == 0: continue
		var new_velocity = initial_direction.rotated(i * angle) * length * 2
		var node = scene.instantiate() as Asteroid
		node.position = location + new_velocity
		node.linear_velocity = new_velocity
		add_child(node)
