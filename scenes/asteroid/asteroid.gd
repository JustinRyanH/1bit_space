class_name Asteroid
extends RigidBody2D

@export var health: int = 3:
	get:
		return health
	set(value):
		health = value
		check_for_death()
@export var next_size: PackedScene
@export var asteroid_spawn_bus: AsteroidSpawnBus

@onready var death_timer := $DeathTimer as Timer

var dying: bool = false


func _ready() -> void:
	var direction = -1 if randi_range(0, 1) == 0 else 1
	angular_velocity = direction * randf_range(1, 3)

func wrap_to(location: Vector2) -> void:
	global_position = location

func take_damage(damage: int) -> void:
	health -= damage

func check_for_death() -> void:
	if dying: return
	if health <= 0:
		dying = true
		death_timer.timeout.connect(destroy_asteroid)
		death_timer.start()

func destroy_asteroid() -> void:
	if next_size:
		asteroid_spawn_bus.spawn_asteroid.emit(next_size, position, linear_velocity * 2, 4)
	queue_free()
