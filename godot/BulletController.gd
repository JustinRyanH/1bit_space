extends Node2D

@export var spawn_bus: BulletSpawnBus

func _ready():
	spawn_bus.connect("spawn_bullet", self.spawn_bullet)

func spawn_bullet(scene_path: String, position: Vector2, rotation: float):
	var scene = load(scene_path)
	var bullet: Node2D = scene.instantiate();
	bullet.global_position = position
	bullet.rotation = rotation
	add_child(bullet)
