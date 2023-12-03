extends Node2D

@export var projectile_stats: ProjectileStates

@onready var hitbox := $Hitbox as Hitbox

func _ready() -> void:
	hitbox.damage = projectile_stats.damage

func _process(delta: float) -> void:
	translate(Vector2.UP.rotated(rotation) * projectile_stats.speed * delta)

func take_damage(damage: int) -> void:
	queue_free()
