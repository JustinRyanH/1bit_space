class_name ProjectileBus
extends Resource

signal create_projectile(
	scene: PackedScene,
	 global_position: Vector2,
	 direction: float,
	 base_speed: float
)
signal add_projectile_to_world(projectile: Node2D);