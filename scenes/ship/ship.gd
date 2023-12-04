extends RigidBody2D

@export var projectile_bus: ProjectileBus
@export var impulse_power: float = 100
@export var rotation_power: float = 20
@export var max_speed: float = 5000
@export var projectile: PackedScene
@onready var gun_position: Node2D = $GunPosition

@onready var engine_particles: = $EngineParticles as GPUParticles2D

var warp_location
var foward_throttle: float = 0.0
var throttle_rotation: float = 0.0

func _process(_delta: float) -> void:
	throttle_rotation = Input.get_axis("Rotate Left", "Rotate Right")
	foward_throttle = Input.get_action_strength("Accelerate")
	if Input.is_action_just_pressed("Fire"):
		var new_projectile = build_projectile()
		projectile_bus.add_projectile_to_world.emit(new_projectile)

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

func take_damage(damage: int, _bdy: Node2D) -> void:
	print("Damage(", name, "): ", damage)

func take_damage_v2(impact_damage: ImpactDamage) -> void:
	take_damage(impact_damage.damage, impact_damage.from)

func build_projectile() -> Projectile:
	var new_projectile := projectile.instantiate() as Projectile
	if not new_projectile:
		printerr("New scene is not a Projecitle")
		return
	new_projectile.setup(Vector2.ZERO, Vector2.UP.rotated(rotation))
	new_projectile.global_position = gun_position.global_position
	new_projectile.global_rotation = gun_position.global_rotation
	new_projectile.add_ignore_targets(self)
	return new_projectile
