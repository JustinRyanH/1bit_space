class_name Asteroid
extends RigidBody2D

@export_group("Signal Buses")
@export var asteroid_spawn_bus: AsteroidSpawnBus
@export var vfx_bus: VfxBus

@export_group("Essential Configuration")
@export var health: int = 3:
	get:
		return health
	set(value):
		health = value
		check_for_death()
@export var next_size: PackedScene

@export_group("Optional Effects")
@export var hit_explosion_particles: PackedScene

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

func take_damage(impact_damage: BasicDamage) -> void:
	spawn_hit_effect(impact_damage)
	if is_invincible: return
	health -= impact_damage.damage

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

func spawn_hit_effect(impact_damage: BasicDamage) -> void:
	if not hit_explosion_particles: return
	if not vfx_bus: return
	var particles := hit_explosion_particles.instantiate() as GPUParticles2D

	
	var space_state = get_world_2d().direct_space_state
	var query = PhysicsRayQueryParameters2D.create(global_position, impact_damage.source_position)
	query.collision_mask = 1
	var result = space_state.intersect_ray(query)
	if result:
		particles.global_position = result["position"]
		particles.global_rotation = Vector2.RIGHT.angle_to(-result["normal"])
	else:
		particles.global_position = global_position
		var hit_direction = Vector2.RIGHT.angle_to((impact_damage.source_position - global_position).normalized())
		particles.global_rotation = hit_direction
	vfx_bus.spawn_particle_gpu.emit(particles)


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
