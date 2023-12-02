class_name AsteroidManager
extends Node2D

@export var asteroid_spawn_bus: AsteroidSpawnBus
@export var spawn_scene: PackedScene
@export var max_asteroid_count: int = 24

func _ready() -> void:
	asteroid_spawn_bus.spawn_asteroid.connect(spawn_sub_asteroid)
	randmize_asteroid_directions()


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

func randmize_asteroid_directions() -> void:
	for child in get_children():
		var asteroid := child as Asteroid
		if not asteroid: continue
		var random_direction = Vector2(randf_range(-1, 1), randf_range(-1, 1)).normalized()
		var random_speed = randi_range(25, 200)
		asteroid.linear_velocity = random_speed * random_direction
