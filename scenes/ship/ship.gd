extends RigidBody2D

@export var projectile_bus: ProjectileBus
@export var impulse_power: float = 100
@export var rotation_power: float = 20
@export var max_speed: float = 5000
@export var projectile: PackedScene

@onready var engine_particles: = $EngineParticles as GPUParticles2D

var warp_location
var foward_throttle: float = 0.0
var throttle_rotation: float = 0.0

func _process(delta: float) -> void:
	throttle_rotation = Input.get_axis("Rotate Left", "Rotate Right")
	foward_throttle = Input.get_action_strength("Accelerate")
	if Input.is_action_just_pressed("Fire"):
		projectile_bus.create_projectile.emit(
			projectile, 
			gun_position.global_position, 
			global_rotation
		)

func _physics_process(delta: float) -> void:
	move_ship(delta)
	engine_particles.emitting = foward_throttle > 0.0

func wrap_to(location: Vector2) -> void:
	global_position = location

func move_ship(delta: float) -> void:
	var direction = Vector2.UP.rotated(rotation)
	
	angular_velocity += throttle_rotation * delta * rotation_power
	linear_velocity += direction * foward_throttle * delta * impulse_power
	linear_velocity = linear_velocity.limit_length(max_speed)

func take_damage(damage: int) -> void:
	print("Damage(", name, "): ", damage)
