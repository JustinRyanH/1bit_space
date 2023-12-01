class_name ShipVFX
extends Node

@export var ship_actor: ShipActor
@export var rear_engine: GPUParticles2D


func _ready():
	rear_engine.emitting = false

func _process(_delta: float):
	if ship_actor.forward_throttle > 0.0:
		rear_engine.emitting = true
	else:
		rear_engine.emitting = false
