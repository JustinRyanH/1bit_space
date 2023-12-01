extends RigidBody2D

@export var impulse_power: float = 100
@export var rotation_power: float = 20
@export var max_speed: float = 5000

var warp_location
var foward_throttle: float = 0.0
var throttle_rotation: float = 0.0

func _process(delta: float) -> void:
	throttle_rotation = Input.get_axis("Rotate Left", "Rotate Right")
	foward_throttle = Input.get_action_strength("Accelerate")
	
func _physics_process(delta: float) -> void:
	move_ship(delta)

func wrap_to(location: Vector2) -> void:
	global_position = location


func move_ship(delta: float) -> void:
	var direction = Vector2.UP.rotated(rotation)
	
	angular_velocity += throttle_rotation * delta * rotation_power
	linear_velocity += direction * foward_throttle * delta * impulse_power
	linear_velocity = linear_velocity.limit_length(max_speed)
