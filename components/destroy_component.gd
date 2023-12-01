class_name DestoryComponent
extends Node

@export var actor: Node2D
@export var destroy_particles: GPUParticles2D
@export var nodes_to_hide: Array

func destroy() -> void:
	for node_path in nodes_to_hide:
		if node_path is NodePath:
			var node := get_node(node_path) as Node2D
			if node:
				node.visible = false
			
	destroy_particles.finished.connect(actor.queue_free)
	destroy_particles.emitting = true
