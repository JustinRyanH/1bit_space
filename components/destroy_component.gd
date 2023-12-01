class_name DestoryComponent
extends Node

@export var actor: Node2D
@export var destroy_particles: GPUParticles2D
@export var move_component: ActorVelocityComponent
@export var nodes_to_hide: Array

var destroyed = false

func destroy() -> void:
	if destroyed: return

	destroyed = true
	for node_path in nodes_to_hide:
		if node_path is NodePath:
			var node := get_node(node_path) as Node2D
			if node:
				node.visible = false
	move_component.velocity = Vector2.ZERO

	destroy_particles.finished.connect(actor.queue_free)
	destroy_particles.emitting = true
