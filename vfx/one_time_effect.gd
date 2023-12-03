extends GPUParticles2D


func _init() -> void:
	one_shot = true


func _on_finished() -> void:
	queue_free()
