extends Node2D

@export var vfx_bus: VfxBus

func _ready() -> void:
    vfx_bus.spawn_particle_gpu.connect(spawn_gpu_particle)

func spawn_gpu_particle(node: GPUParticles2D) -> void:
    if not node: return
    node.emitting = true
    add_child(node)