class_name ActorVelocityComponent
extends Node

@export var actor: Node2D
@export var velocity: Vector2 = Vector2.ZERO

@onready var actor_velocity_component: ActorVelocityComponent = $ActorVelocityComponent

func _process(delta: float) -> void:
	actor.translate(velocity * delta)
