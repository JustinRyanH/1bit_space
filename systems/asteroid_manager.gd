class_name AsteroidManager
extends Node2D

const EDGE_OFFSET = 25
const EDGE_OUTBOUND = 200

enum Side {
	TOP = 0,
	RIGHT = 1,
	BOTTOM = 2,
	LEFT = 3,
}

@export var asteroid_spawn_bus: AsteroidSpawnBus
@export var spawn_scene: PackedScene
@export var max_asteroid_count: int = 24

@onready var world_wrap := %WorldWrap as WorldWrap


func _ready() -> void:
	asteroid_spawn_bus.spawn_asteroids.connect(spawn_sub_asteroid)
	asteroid_spawn_bus.completely_destroyed.connect(spawn_new_asteroid)
	# randmize_asteroid_directions()

func spawn_sub_asteroid(scene: PackedScene, location: Vector2, velocity: Vector2) -> void:
	var node = scene.instantiate() as Asteroid
	node.position = location
	node.linear_velocity = velocity
	add_child(node)
		
func spawn_new_asteroid() -> void:
	var total_children = get_child_count()
	if total_children >= max_asteroid_count: return
	var random_side := randi_range(0, 3) as Side
	
	var new_position = get_random_position_from_direction(random_side)
	create_a_asteroid(new_position)

func randmize_asteroid_directions() -> void:
	for child in get_children():
		var asteroid := child as Asteroid
		if not asteroid: continue
		var random_direction = Vector2(randf_range(-1, 1), randf_range(-1, 1)).normalized()
		var random_speed = randi_range(25, 200)
		asteroid.linear_velocity = random_speed * random_direction

func get_random_position_from_direction(direction: Side) -> Vector2:
	var bounds = world_wrap.get_bounds()
	
	match direction:
		Side.TOP:
			var y = bounds.position.y - EDGE_OUTBOUND
			var x = random_x(bounds)
			return Vector2(x, y)
		Side.LEFT:
			var y = random_y(bounds)
			var x = bounds.position.x - EDGE_OUTBOUND
			return Vector2(x, y)
		Side.RIGHT:
			var y = random_y(bounds)
			var x = bounds.size.x * 0.5 + EDGE_OUTBOUND
			return Vector2(x, y)
		Side.BOTTOM:
			var y = bounds.size.y * 0.5 + EDGE_OUTBOUND
			var x = random_x(bounds)
			return Vector2(x, y)
	return Vector2.ZERO

func random_angle() -> float:
	return randf_range(-PI / 8, PI / 8)

func random_x(bounds: Rect2) -> float:
	return randf_range(EDGE_OFFSET + bounds.position.x, (bounds.size.x / 2) - EDGE_OFFSET)
	
func random_y(bounds: Rect2) -> float:
	return randf_range(EDGE_OFFSET + bounds.position.y, (bounds.size.y / 2) - EDGE_OFFSET)

func create_a_asteroid(new_position: Vector2) -> void:
	var random_speed = randf_range(100, 300)
	var node = spawn_scene.instantiate() as Asteroid
	
	node.position = new_position
	var direction = (Vector2.ZERO - node.position).normalized().rotated(random_angle())
	
	node.linear_velocity = direction * random_speed
	add_child(node)
