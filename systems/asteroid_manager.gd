class_name AsteroidManager
extends Node2D

const EDGE_OFFSET = 25

enum Side {
	Top = 0,
	Right = 1,
	Bottom = 2,
	Left = 3,
}

@export var asteroid_spawn_bus: AsteroidSpawnBus
@export var spawn_scene: PackedScene
@export var max_asteroid_count: int = 24

@onready var world_wrap := %WorldWrap as WorldWrap


func _ready() -> void:
	asteroid_spawn_bus.spawn_asteroid.connect(spawn_sub_asteroid)
	asteroid_spawn_bus.completely_destroyed.connect(spawn_new_asteroid)
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
		node.position = location + new_velocity * 0.5
		node.linear_velocity = new_velocity
		add_child(node)
		
func spawn_new_asteroid() -> void:
	var total_children = get_child_count()
	if total_children >= max_asteroid_count: return
	var random_side: Side = randi_range(0, 3)

	spawn_random_bottom()

func randmize_asteroid_directions() -> void:
	for child in get_children():
		var asteroid := child as Asteroid
		if not asteroid: continue
		var random_direction = Vector2(randf_range(-1, 1), randf_range(-1, 1)).normalized()
		var random_speed = randi_range(25, 200)
		asteroid.linear_velocity = random_speed * random_direction
		
func spawn_random_top() -> void:
	var bounds = world_wrap.get_bounds()
	
	var random_speed = randf_range(50, 200)
	var y = bounds.position.y - 50
	var x = random_x(bounds)
	var node = spawn_scene.instantiate() as Asteroid
	node.linear_velocity = Vector2.DOWN.rotated(random_angle()) * random_speed
	node.position = Vector2(x, y)
	add_child(node)
	
func spawn_random_bottom() -> void:
	var bounds = world_wrap.get_bounds()
	
	var y = bounds.size.y * 0.5 + 200
	var x = random_x(bounds)
	var new_position = Vector2(x, y)

	create_a_asteroid(new_position)


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
	node.modulate.r = 0
	add_child(node)
