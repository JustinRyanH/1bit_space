class_name ProjectileMoveComponent
extends Node

@export var actor: Node2D
@export var projectile_stats: ProjectileStates


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var velocity = Vector2.UP.rotated(actor.rotation) * delta * projectile_stats.speed
	actor.translate(velocity)
