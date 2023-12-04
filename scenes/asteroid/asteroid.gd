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
@onready var incincible_timer: Timer = $IncincibleTimer
@onready var visual_despawner: Timer = $VisualDespawner
@onready var spawn_positions: Node2D = $SpawnPositions

var is_invincible := true

var dying: bool = false

func _ready() -> void:
	var direction = -1 if randi_range(0, 1) == 0 else 1
	angular_velocity = direction * randf_range(1, 3)
	incincible_timer.timeout.connect(func(): is_invincible = false)
	
	
func wrap_to(location: Vector2) -> void:
	global_position = location

func take_damage(damage: int, _by: Node2D) -> void:
	if is_invincible: return
	
	health -= damage

func take_damage_v2(impact_damage: ImpactDamage) -> void:
	take_damage(impact_damage.damage, impact_damage.from)

func check_for_death() -> void:
	if dying: return
	if health <= 0:
		dying = true
		death_timer.timeout.connect(destroy_asteroid)
		death_timer.start()

func destroy_asteroid() -> void:
	if next_size:
		var normalized_velocity := linear_velocity.normalized()
		var speed := linear_velocity.length() * 1.5;
		for point in spawn_positions.get_children():
			var next_position = point.global_position
			var next_rotation = point.rotation
			var new_direction = normalized_velocity.rotated(next_rotation)
			asteroid_spawn_bus.spawn_asteroids.emit(next_size, next_position, new_direction * speed)
	else:
		asteroid_spawn_bus.completely_destroyed.emit()
	queue_free()


func _on_input_event(viewport: Node, event: InputEvent, shape_idx: int) -> void:
	if event is InputEventMouseButton:
		var mouse_button := event as InputEventMouseButton
		if mouse_button.is_action_pressed("Mouse Left"):
			health = 0


func _on_visible_on_screen_notifier_2d_screen_entered() -> void:
	visual_despawner.stop()


func _on_visual_despawner_timeout() -> void:
	asteroid_spawn_bus.completely_destroyed.emit()
	queue_free()
