class_name ShipMovement
extends Node

@export var ship_actor: ShipActor
@export var velocity_component: ActorVelocityComponent

func _process(delta: float) -> void:
	ship_rotate(delta)
	ship_accelerate(delta)

func ship_rotate(delta: float) -> void:
	pass
	#var configuration: ShipConfiguration = ship_actor.configuration
	#ship_actor.rotation += (ship_actor.rotation_throttle * delta * configuration.turn_rate)
	
func ship_accelerate(delta: float) -> void:
	var configuration: ShipConfiguration = ship_actor.configuration
	velocity_component.velocity += (Vector2.UP.rotated(ship_actor.rotation) * delta * configuration.impulse_power * ship_actor.forward_throttle)
	velocity_component.velocity = velocity_component.velocity.limit_length(configuration.max_speed)
