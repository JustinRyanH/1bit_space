extends Node2D

@export var projectile_bus: ProjectileBus

func _ready():
	projectile_bus.add_projectile_to_world.connect(insert_projectile)
	
func insert_projectile(new_projectile: Node2D) -> void:
	var projectile := new_projectile as Projectile;
	if projectile: add_child(projectile)

